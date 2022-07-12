use crate::{is_aligned, U32BigEndian};
use core::ops::Range;

pub(crate) struct FdtHeader {
    magic: U32BigEndian,
    pub totalsize: U32BigEndian,
    pub off_dt_struct: U32BigEndian,
    pub off_dt_strings: U32BigEndian,
    pub off_mem_rsvmap: U32BigEndian,
    pub version: U32BigEndian,
    pub last_comp_version: U32BigEndian,
    #[allow(unused)]
    pub boot_cpuid_phys: U32BigEndian,
    pub size_dt_strings: U32BigEndian,
    pub size_dt_struct: U32BigEndian,
}

/// 首部检查可能发现的错误类型。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum HeaderError {
    /// 设备树整体不对齐。
    Misaligned(u32),
    /// `magic` 字段不是 0xd00dfeed。
    Magic(u32),
    /// 版本不兼容。
    Version(u32),
    /// 最后兼容版本不兼容。
    LastCompVersion(u32),
    /// 设备树总大小不合理。
    TotalSize(u32),
    /// 结构块偏移不对齐。
    StructMisaligned(u32),
    /// 结构块偏移不合理。
    StructOffset {
        /// 解析出的结构块偏移。
        value: u32,
        /// 可接受的结构块偏移。
        expected: Range<u32>,
    },
    /// 结构块大小不合理。
    StructSize {
        /// 解析出的结构块大小。
        value: u32,
        /// 可接受的最大结构块。
        max: u32,
    },
    /// 结构块内容不合理，即没有根节点或不以 END 标记结尾。
    StructContent,
    /// 地址保护区偏移不对齐。
    MemRsvMisaligned(u32),
    /// 地址保护区偏移不合理。
    MemRsvOffset {
        /// 解析出的地址保护区偏移。
        value: u32,
        /// 可接受的地址保护区偏移。
        expected: Range<u32>,
    },
    /// 字符串区偏移不合理。
    StringsOffset {
        /// 解析出的字符串区偏移。
        value: u32,
        /// 可接受的字符串区偏移。
        expected: Range<u32>,
    },
    /// 字符串区大小不合理。
    StringsSize {
        /// 解析出的字符串区大小。
        value: u32,
        /// 可接受的字符串区大小。
        max: u32,
    },
}

const FOUR: usize = 4;
const DTB_ALIGN_BITS: usize = 8;
const MEMREV_ALIGN_BITS: usize = FOUR;
const STRUCT_ALIGN_BITS: usize = FOUR;
const STRUCT_SIZE_ALIGN_BITS: usize = FOUR;

const MAGIC: U32BigEndian = U32BigEndian::from_u32(0xd00dfeed);
const VERSION: u32 = 17;
const LAST_COMP_VERSION: u32 = 16;
const LEN_HEADER: u32 = core::mem::size_of::<FdtHeader>() as _;

impl FdtHeader {
    pub fn verify(&self, filter: impl Fn(&HeaderError) -> bool) -> Result<(), HeaderError> {
        use HeaderError as E;
        // 检查整体对齐
        if !is_aligned(self as *const _ as _, DTB_ALIGN_BITS) {
            check(&filter, E::Misaligned(misaligned(self as *const _ as _)))?;
        }
        // 检查 magic 和版本
        if self.magic != MAGIC {
            check(&filter, E::Magic(self.magic.into_u32()))?;
        }
        if self.version.into_u32() < VERSION {
            check(&filter, E::Version(self.version.into_u32()))?;
        }
        if self.last_comp_version.into_u32() != LAST_COMP_VERSION {
            check(
                &filter,
                E::LastCompVersion(self.last_comp_version.into_u32()),
            )?;
        }
        // 检查结构
        let len_total = self.totalsize.into_u32();
        if len_total < LEN_HEADER {
            check(&filter, E::TotalSize(len_total))?;
        }
        let mut range = LEN_HEADER..len_total;
        // 保留内存块
        let off_memrev = self.off_mem_rsvmap.into_u32();
        if !is_aligned(off_memrev as _, MEMREV_ALIGN_BITS) {
            check(&filter, E::MemRsvMisaligned(misaligned(off_memrev)))?;
        }
        if !range.contains(&off_memrev) {
            check(
                &filter,
                E::MemRsvOffset {
                    value: off_memrev,
                    expected: range.clone(),
                },
            )?;
        }
        range = off_memrev..len_total;
        // 结构块
        let off_struct = self.off_dt_struct.into_u32();
        if !is_aligned(off_struct as _, STRUCT_ALIGN_BITS) {
            check(&filter, E::StructMisaligned(misaligned(off_struct)))?;
        }
        if !range.contains(&off_struct) {
            check(
                &filter,
                E::StructOffset {
                    value: off_struct,
                    expected: range.clone(),
                },
            )?;
        }
        let len_struct = self.size_dt_struct.into_u32();
        if !is_aligned(len_struct as _, STRUCT_SIZE_ALIGN_BITS) {
            check(&filter, E::StructMisaligned(misaligned(len_struct)))?;
        }
        if len_struct > range.len() as u32 {
            check(
                &filter,
                E::StructSize {
                    value: len_struct,
                    max: range.len() as _,
                },
            )?;
        }
        unsafe {
            use crate::StructureBlock as Blk;
            match core::slice::from_raw_parts(
                (self as *const _ as *const u8)
                    .offset(off_struct as _)
                    .cast::<Blk>(),
                len_struct as usize / Blk::LEN,
            ) {
                [Blk::NODE_BEGIN, Blk::EMPTY_STR, .., Blk::END] => {}
                _ => check(&filter, E::StructContent)?,
            }
        }
        range = off_struct + len_struct..len_total;
        // 字符串块
        let off_strings = self.off_dt_strings.into_u32();
        if !range.contains(&off_strings) {
            check(
                &filter,
                E::StringsOffset {
                    value: off_strings,
                    expected: range.clone(),
                },
            )?;
        }
        let len_strings = self.size_dt_strings.into_u32();
        if len_strings > range.len() as u32 {
            check(
                filter,
                E::StringsSize {
                    value: len_strings,
                    max: range.len() as _,
                },
            )?;
        }
        Ok(())
    }
}

#[inline]
fn misaligned(addr: u32) -> u32 {
    1 << addr.trailing_zeros()
}

#[inline]
fn check(filter: impl Fn(&HeaderError) -> bool, err: HeaderError) -> Result<(), HeaderError> {
    if filter(&err) {
        Ok(())
    } else {
        Err(err)
    }
}
