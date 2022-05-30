#![no_std]
#![feature(slice_internals)]

use core::{fmt, slice};

mod header;
mod path;
mod reg;
mod structure_block;
mod walker;

pub use path::Path;
pub use reg::Reg;

use header::{FdtHeader, HeaderError};
use reg::RegCfg;
use structure_block::StructureBlock;
use walker::Walker;

/// 设备树二进制对象。
pub struct Dtb<'a>(&'a [u8]);

impl Dtb<'static> {
    /// 构造设备树二进制对象引用。
    ///
    /// # Safety
    ///
    /// 如果指针指向一个有效的 DTB 首部，其中描述的整个二进制对象会被切片。
    pub unsafe fn from_raw_parts(ptr: *const u8) -> Result<Self, HeaderError> {
        let header: &FdtHeader = &*ptr.cast();
        header.verify()?;
        Ok(Self(slice::from_raw_parts(
            ptr,
            header.totalsize.into_u32() as _,
        )))
    }

    /// 返回整个二进制对象的尺寸。
    #[inline]
    pub const fn total_size(&self) -> usize {
        self.0.len()
    }

    /// 遍历。
    pub fn walk(&self, f: &mut impl FnMut(&Path<'_>, DtbObj) -> WalkOperation) {
        let header = self.header();
        let off_struct = header.off_dt_struct.into_u32() as usize;
        let len_struct = header.size_dt_struct.into_u32() as usize;
        let off_strings = header.off_dt_strings.into_u32() as usize;
        let len_strings = header.size_dt_strings.into_u32() as usize;
        Walker {
            tail: unsafe {
                slice::from_raw_parts(
                    self.0[off_struct..]
                        .as_ptr()
                        .cast::<StructureBlock>()
                        .offset(2),
                    len_struct / StructureBlock::LEN - 3,
                )
            },
            strings: &self.0[off_strings..][..len_strings],
        }
        .walk_inner(f, Path::root(), RegCfg::DEFAULT, false);
    }

    #[inline]
    fn header(&self) -> &FdtHeader {
        unsafe { &*self.0.as_ptr().cast() }
    }
}

/// 设备树二进制小对象。
pub enum DtbObj<'a> {
    /// 子节点
    SubNode { name: &'a [u8] },
    /// 一般属性
    Property { name: &'a [u8], value: &'a [u8] },
    /// 寄存器属性
    Reg(Reg<'a>),
}

pub enum WalkOperation {
    /// 进入子节点
    StepInto,
    /// 跳过子节点
    StepOver,
    /// 跳过当前子树
    StepOut,
    /// 结束遍历
    Terminate,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
struct U32BigEndian(u32);

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

impl fmt::Debug for U32BigEndian {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        u32::from_be(self.0).fmt(f)
    }
}

#[inline]
fn is_aligned(val: usize, bits: usize) -> bool {
    val & (bits - 1) == 0
}
