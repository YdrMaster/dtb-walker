use core::fmt;

mod reg;
mod str;

use crate::StructureBlock;

pub use self::str::{Str, StrList};
pub use reg::Reg;
pub(crate) use reg::RegCfg;

pub struct PHandle(u32);

impl PHandle {
    pub(crate) fn new(value: &[StructureBlock]) -> Self {
        match *value {
            [blk] => Self(blk.into_u32()),
            _ => panic!(),
        }
    }
}

impl fmt::Debug for PHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<")?;
        self.0.fmt(f)?;
        write!(f, ">")
    }
}
