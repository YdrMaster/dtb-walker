use super::{Error, Result as Res};
use crate::StructureBlock;
use core::fmt;

pub struct PHandle(u32);

impl PHandle {
    #[inline]
    pub(super) fn new(value: &[StructureBlock]) -> Res<Self> {
        match *value {
            [blk] => Ok(Self(blk.into_u32())),
            _ => Err(Error),
        }
    }

    #[inline]
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl fmt::Debug for PHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<")?;
        self.0.fmt(f)?;
        write!(f, ">")
    }
}
