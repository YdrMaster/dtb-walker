//! §2.3

mod phandle;
mod reg;
mod str;

use crate::{Str, StructureBlock};
use core::{fmt, slice};

pub use self::phandle::PHandle;
pub use self::str::StrList;
pub use reg::Reg;
pub(crate) use reg::RegCfg;

/// 属性
pub enum Property<'a> {
    /// §2.3.1 兼容性
    Compatible(StrList<'a>),
    /// §2.3.2 型号
    Model(Str<'a>),
    /// §2.3.3 引用号
    PHandle(PHandle),
    /// §2.3.4 状态
    Status(Str<'a>),
    /// §2.3.6 寄存器
    Reg(Reg<'a>),
    /// §2.3.7 寄存器
    VirtualReg(u32),
    /// §2.3.10 DMA 连贯性
    DmaCoherent,
    /// 一般属性
    General {
        /// 属性名
        name: Str<'a>,
        /// 属性值
        value: &'a [u8],
    },
}

struct Error;

type Result<T> = core::result::Result<T, Error>;

impl<'a> Property<'a> {
    pub(crate) fn new(name: &'a [u8], value: &'a [StructureBlock], len: usize) -> Self {
        let general = |Error| Self::General {
            name: Str(name),
            value: unsafe { slice::from_raw_parts(value.as_ptr().cast(), len) },
        };
        match name {
            b"compatible" => StrList::new(value, len).map_or_else(general, Self::Compatible),
            b"model" => Str::new(value, len).map_or_else(general, Self::Model),
            b"phandle" | b"linux,phandle" => u32_from(value)
                .map(PHandle)
                .map_or_else(general, Self::PHandle),
            b"status" => Str::new(value, len).map_or_else(general, Self::Status),
            b"virtual-reg" => u32_from(value).map_or_else(general, Self::VirtualReg),
            b"dma-coherent" if value.is_empty() => Self::DmaCoherent,
            _ => general(Error),
        }
    }
}

impl fmt::Debug for Property<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compatible(compatible) => write!(f, "compatible = {compatible};"),
            Self::Model(model) => write!(f, "model = {model};"),
            Self::Reg(reg) => write!(f, "reg = {reg:#x?};"),
            Self::PHandle(phandle) => write!(f, "phandle = {phandle:?};"),
            Self::Status(status) => write!(f, "status = {status};"),
            Self::VirtualReg(vreg) => {
                write!(f, "virtual-reg = <")?;
                vreg.fmt(f)?;
                write!(f, ">;")
            }
            Self::DmaCoherent => write!(f, "dma-coherent;"),
            Self::General { name, value } => {
                write!(f, "{}", unsafe { name.as_str_unchecked() })?;
                match name {
                    _ if !value.is_empty() => {
                        write!(f, " = {value:02x?};")
                    }
                    _ => {
                        write!(f, ";")
                    }
                }
            }
        }
    }
}

#[inline]
fn u32_from(value: &[StructureBlock]) -> Result<u32> {
    match *value {
        [blk] => Ok(blk.into_u32()),
        _ => Err(Error),
    }
}
