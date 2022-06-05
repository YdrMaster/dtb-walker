#![no_std]
#![feature(slice_internals)]

use core::{fmt, mem, slice};

mod header;
mod indent;
mod path;
mod property;
mod structure_block;
mod walker;

pub use path::Path;
pub use property::{PHandle, Property, Reg, Str, StrList};
pub mod utils {
    pub use crate::indent::indent;
}

use header::{FdtHeader, HeaderError};
use property::RegCfg;
use structure_block::StructureBlock;
use walker::Walker;

/// 设备树二进制对象。
pub struct Dtb<'a>(&'a [u8]);

impl Dtb<'static> {
    /// 构造设备树二进制对象。
    ///
    /// # Safety
    ///
    /// 如果指针指向一个有效的 DTB 首部，其中描述的整个二进制对象会被切片。
    #[inline]
    pub unsafe fn from_raw_parts(ptr: *const u8) -> Result<Self, HeaderError> {
        (*ptr.cast::<FdtHeader>()).verify()?;
        Ok(Self::from_raw_parts_unchecked(ptr))
    }

    /// 不检查首部正确性，直接构造设备树二进制对象。
    ///
    /// # Safety
    ///
    /// 假设指针指向一个正确的设备树二进制对象，其首部描述的整个二进制对象会被切片。
    #[inline]
    pub unsafe fn from_raw_parts_unchecked(ptr: *const u8) -> Self {
        Self(slice::from_raw_parts(
            ptr,
            (*ptr.cast::<FdtHeader>()).totalsize.into_u32() as _,
        ))
    }
}

pub enum ConvertError {
    Truncated,
    Header(HeaderError),
}

impl<'a> Dtb<'a> {
    /// 从内存切片安全地创建设备树二进制对象。
    pub fn from_slice(slice: &'a [u8]) -> Result<Self, ConvertError> {
        if slice.len() < mem::size_of::<FdtHeader>() {
            return Err(ConvertError::Truncated);
        }
        let header = unsafe { &*slice.as_ptr().cast::<FdtHeader>() };
        match header.verify() {
            Ok(()) => {
                let len = header.totalsize.into_u32() as usize;
                if len <= slice.len() {
                    Ok(Self(&slice[..len]))
                } else {
                    Err(ConvertError::Truncated)
                }
            }
            Err(e) => Err(ConvertError::Header(e)),
        }
    }
}

impl Dtb<'_> {
    /// 返回整个二进制对象的尺寸。
    #[inline]
    pub const fn total_size(&self) -> usize {
        self.0.len()
    }

    /// 遍历。
    pub fn walk(&self, mut f: impl FnMut(&Path<'_>, DtbObj) -> WalkOperation) {
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
        .walk_inner(&mut f, &Path::ROOT, RegCfg::DEFAULT, false);
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
    Property(Property<'a>),
}

/// 遍历操作。
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
