use core::ops::Range;

use crate::{is_aligned, U32BigEndian};

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

#[derive(Debug)]
pub enum HeaderError {
    Misaligned(u32),
    Magic(u32),
    Version(u32),
    LastCompVersion(u32),
    TotalSize(u32),
    StructMisaligned(u32),
    StructOffset { value: u32, expected: Range<u32> },
    StructSize { value: u32, max: u32 },
    StructContent,
    MemRsvMisaligned(u32),
    MemRsvOffset { value: u32, expected: Range<u32> },
    StringsOffset { value: u32, expected: Range<u32> },
    StringsSize { value: u32, max: u32 },
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
    filter(&err).then_some(()).ok_or(err)
}
