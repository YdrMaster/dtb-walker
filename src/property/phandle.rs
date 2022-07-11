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
