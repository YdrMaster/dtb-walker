use core::{fmt, str};

/// 地址空间上的一个字符串，但未检查是否符合 utf-8 编码。
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Str<'a>(pub(crate) &'a [u8]);

impl Str<'_> {
    /// Converts to `&[u8]`.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0
    }

    /// Converts to [`str`].
    #[inline]
    pub fn as_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(self.0)
    }

    /// Converts to [`str`] without checking utf-8 validity.
    ///
    /// # Safety
    ///
    /// see [`core::str::from_utf8_unchecked`].
    #[inline]
    pub unsafe fn as_str_unchecked(&self) -> &str {
        str::from_utf8_unchecked(self.0)
    }

    /// Returns `true` if `needle` is a prefix of this string.
    #[inline]
    pub fn starts_with(&self, needle: &str) -> bool {
        self.0.starts_with(needle.as_bytes())
    }
}

impl<'a> From<&'a str> for Str<'a> {
    #[inline]
    fn from(s: &'a str) -> Self {
        Str(s.as_bytes())
    }
}

impl fmt::Display for Str<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { self.as_str_unchecked() }.fmt(f)
    }
}
