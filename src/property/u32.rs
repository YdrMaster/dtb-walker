use super::{Error, Result as Res};
use crate::StructureBlock;
use core::fmt;

pub struct PHandle(pub(super) u32);

impl PHandle {
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

#[inline]
pub(super) fn u32_from(value: &[StructureBlock]) -> Res<u32> {
    match *value {
        [blk] => Ok(blk.into_u32()),
        _ => Err(Error),
    }
}
