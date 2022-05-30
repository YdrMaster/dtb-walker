use crate::{is_aligned, U32BigEndian};

pub(crate) struct FdtHeader {
    pub magic: U32BigEndian,
    pub totalsize: U32BigEndian,
    pub off_dt_struct: U32BigEndian,
    pub off_dt_strings: U32BigEndian,
    pub off_mem_rsvmap: U32BigEndian,
    pub version: U32BigEndian,
    pub last_comp_version: U32BigEndian,
    pub boot_cpuid_phys: U32BigEndian,
    pub size_dt_strings: U32BigEndian,
    pub size_dt_struct: U32BigEndian,
}

#[derive(Debug)]
pub enum HeaderError {
    Misaligned,
    Magic,
    Version,
    LastCompVersion,
    TotalSize,
    StructMisaligned,
    StructOffset,
    StructSize,
    StructContent,
    MemRsvMisaligned,
    MemRsvOffset,
    StringsOffset,
    StringsSize,
}

const DTB_ALIGN_BITS: usize = core::mem::size_of::<usize>();
const MEMREV_ALIGN_BITS: usize = 4;
const STRUCT_ALIGN_BITS: usize = 4;
const STRUCT_SIZE_ALIGN_BITS: usize = 4;

const MAGIC: U32BigEndian = U32BigEndian::from_u32(0xd00dfeed);
const VERSION: u32 = 17;
const LAST_COMP_VERSION: u32 = 16;
const LEN_HEADER: u32 = core::mem::size_of::<FdtHeader>() as _;

impl FdtHeader {
    // pub fn body_len(&self) ->usize{
    //     self.totalsize-
    // }

    pub fn verify(&self) -> Result<(), HeaderError> {
        use HeaderError as E;
        // 检查整体对齐
        if !is_aligned(self as *const _ as _, DTB_ALIGN_BITS) {
            return Err(E::Misaligned);
        }
        // 检查 magic 和版本
        if self.magic != MAGIC {
            return Err(E::Magic);
        }
        if self.version.into_u32() < VERSION {
            return Err(E::Version);
        }
        if self.last_comp_version.into_u32() != LAST_COMP_VERSION {
            return Err(E::LastCompVersion);
        }
        // 检查结构
        let len_total = self.totalsize.into_u32();
        if len_total < LEN_HEADER {
            return Err(E::TotalSize);
        }
        let mut range = LEN_HEADER..len_total;
        // 保留内存块
        let off_memrev = self.off_mem_rsvmap.into_u32();
        if !is_aligned(off_memrev as _, MEMREV_ALIGN_BITS) {
            return Err(E::MemRsvMisaligned);
        }
        if !range.contains(&off_memrev) {
            return Err(E::MemRsvOffset);
        }
        range = off_memrev..len_total;
        // 结构块
        let off_struct = self.off_dt_struct.into_u32();
        if !is_aligned(off_struct as _, STRUCT_ALIGN_BITS) {
            return Err(E::StructMisaligned);
        }
        if !range.contains(&off_struct) {
            return Err(E::StructOffset);
        }
        let len_struct = self.size_dt_struct.into_u32();
        if !is_aligned(len_struct as _, STRUCT_SIZE_ALIGN_BITS) {
            return Err(E::StructMisaligned);
        }
        if len_struct > range.len() as u32 {
            return Err(E::StructSize);
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
                _ => return Err(E::StructContent),
            }
        }
        range = off_struct + len_struct..len_total;
        // 字符串块
        let off_strings = self.off_dt_strings.into_u32();
        if !range.contains(&off_strings) {
            return Err(E::StringsOffset);
        }
        let len_strings = self.size_dt_strings.into_u32();
        if len_strings > range.len() as u32 {
            return Err(E::StringsSize);
        }
        Ok(())
    }
}
