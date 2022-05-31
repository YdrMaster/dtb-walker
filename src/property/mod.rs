mod reg;
mod str;
mod u32;

use crate::StructureBlock;
use core::slice;

pub use self::str::{Str, StrList};
pub use self::u32::PHandle;
pub use reg::Reg;
pub(crate) use reg::RegCfg;

/// 属性
pub enum Property<'a> {
    /// 兼容性
    Compatible(StrList<'a>),
    /// 型号
    Model(Str<'a>),
    /// 寄存器
    Reg(Reg<'a>),
    /// 引用号
    PHandle(PHandle),
    /// 一般属性
    General { name: &'a [u8], value: &'a [u8] },
}

struct Error;
type Result<T> = core::result::Result<T, Error>;

impl<'a> Property<'a> {
    pub(crate) fn new(name: &'a [u8], value: &'a [StructureBlock], len: usize) -> Self {
        let general = |Error| Self::General {
            name,
            value: unsafe { slice::from_raw_parts(value.as_ptr().cast(), len) },
        };
        match name {
            b"compatible" => StrList::new(value, len).map_or_else(general, Self::Compatible),
            b"model" => Str::new(value, len).map_or_else(general, Self::Model),
            b"phandle" | b"linux,phandle" => {
                PHandle::new(value).map_or_else(general, Self::PHandle)
            }
            _ => general(Error),
        }
    }
}
