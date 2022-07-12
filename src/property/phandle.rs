//! §2.3.3

use core::fmt;

/// §2.3.3 phandle 属性
pub struct PHandle(pub(super) u32);

impl PHandle {
    /// 返回 phandle 值。
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
