#![no_std]

mod header;
mod structure_block;

use header::{FdtHeader, HeaderError};
use structure_block::StructureBlock;

/// 设备树递归结构。
pub struct DtbWalker<'a> {
    tail: &'a [StructureBlock],
    context: DtbContext,

    header: &'a FdtHeader,
    strings: &'a [u8],
}

impl DtbWalker<'static> {
    /// 构造设备树二进制对象递归遍历上下文。
    ///
    /// # Safety
    ///
    /// 如果指针指向一个有效的 DTB 首部，其中描述的各个数据段会被切片。
    pub unsafe fn new(ptr: *const u8) -> Result<Self, HeaderError> {
        let header: &FdtHeader = &*ptr.cast();
        header.verify()?;
        Ok(Self {
            tail: core::slice::from_raw_parts(
                ptr.offset(header.off_dt_struct.into_u32() as _)
                    .cast::<StructureBlock>()
                    .offset(2),
                (header.size_dt_struct.into_u32() as usize) / StructureBlock::LEN - 3,
            ),
            context: DtbContext::DEFAULT,

            header,
            strings: core::slice::from_raw_parts(
                ptr.offset(header.off_dt_strings.into_u32() as _),
                header.size_dt_strings.into_u32() as _,
            ),
        })
    }
}

struct DtbContext {
    address_cells: u32,
    size_cells: u32,
}

impl DtbContext {
    const DEFAULT: Self = Self {
        address_cells: 2,
        size_cells: 1,
    };
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct U32BigEndian(u32);

impl U32BigEndian {
    #[inline]
    pub const fn from_u32(val: u32) -> Self {
        Self(u32::to_be(val))
    }

    #[inline]
    pub const fn into_u32(self) -> u32 {
        u32::from_be(self.0)
    }
}

#[inline]
fn is_aligned(val: usize, bits: usize) -> bool {
    val & (bits - 1) == 0
}
