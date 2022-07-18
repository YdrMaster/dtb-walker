//! A simple package for DTB depth-first walking.
//!
//! # Example
//!
//! ```cmd
//! cargo run --release --example qemu-virt
//! ```
//!
//! # Usage
//!
//! ```rust,no_run
//! Dtb::from_raw_parts(dtb).unwrap()
//! ```

#![no_std]
#![deny(warnings, unstable_features, missing_docs)] // cancel this line during developing

mod context;
mod header;
mod indent;
mod property;
mod str;
mod structure_block;
mod tree_on_stack;
mod walker;

pub use self::str::Str;
pub use property::{PHandle, Property, Reg, StrList};
pub mod utils {
    //! 用于设备树解析、格式化的工具集。

    pub use crate::indent::indent;
}
pub use context::{Context, ContextMeta};
pub use header::HeaderError;

use core::{fmt, mem, slice};
use header::FdtHeader;
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
        (*ptr.cast::<FdtHeader>()).verify(|_| false)?;
        Ok(Self::from_raw_parts_unchecked(ptr))
    }

    /// 构造设备树二进制对象，并可以选择接受某些不合规范的情况。
    ///
    /// # Safety
    ///
    /// 如果指针指向一个有效的 DTB 首部，其中描述的整个二进制对象会被切片。
    #[inline]
    pub unsafe fn from_raw_parts_filtered(
        ptr: *const u8,
        f: impl Fn(&HeaderError) -> bool,
    ) -> Result<Self, HeaderError> {
        (*ptr.cast::<FdtHeader>()).verify(f)?;
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

/// 从内存切片构造设备树二进制对象失败。
pub enum ConvertError {
    /// 首部检查未通过。
    Header(HeaderError),
    /// 切片未能容纳整个设备树。
    Truncated,
}

impl<'a> Dtb<'a> {
    /// 从内存切片安全地创建设备树二进制对象，可以选择接受某些不合规范的情况。
    pub fn from_slice_filtered(
        slice: &'a [u8],
        f: impl Fn(&HeaderError) -> bool,
    ) -> Result<Self, ConvertError> {
        if slice.len() < mem::size_of::<FdtHeader>() {
            return Err(ConvertError::Truncated);
        }
        let header = unsafe { &*slice.as_ptr().cast::<FdtHeader>() };
        match header.verify(f) {
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

    /// 从内存切片安全地创建设备树二进制对象。
    #[inline]
    pub fn from_slice(slice: &'a [u8]) -> Result<Self, ConvertError> {
        Self::from_slice_filtered(slice, |_| false)
    }
}

impl Dtb<'_> {
    /// 返回整个二进制对象的尺寸。
    #[inline]
    pub const fn total_size(&self) -> usize {
        self.0.len()
    }

    /// 遍历。
    pub fn walk<Meta: ContextMeta>(&self, root_meta: Meta) -> Meta {
        let header = self.header();
        let off_struct = header.off_dt_struct.into_u32() as usize;
        let len_struct = header.size_dt_struct.into_u32() as usize;
        let off_strings = header.off_dt_strings.into_u32() as usize;
        let len_strings = header.size_dt_strings.into_u32() as usize;
        let mut root = Context::root(root_meta);
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
        .walk_inner(&mut root);
        root.0.data.meta
    }

    #[inline]
    fn header(&self) -> &FdtHeader {
        unsafe { &*self.0.as_ptr().cast() }
    }
}

/// 遍历操作。
pub enum WalkOperation<T> {
    /// 进入子节点。
    Access(T),
    /// 跳过子节点。
    Skip(SkipType),
}

/// 跳过子节点的方式。
pub enum SkipType {
    /// 跳过子节点。
    StepOver,
    /// 跳过当前子树。
    StepOut,
    /// 结束遍历。
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
