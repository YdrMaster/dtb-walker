use crate::StructureBlock;
use core::{fmt, slice, str};

#[derive(Clone)]
pub struct Str<'a>(pub &'a [u8]);

impl<'a> Str<'a> {
    #[inline]
    pub(crate) fn new(value: &'a [StructureBlock], len: usize) -> Self {
        let buf = unsafe { slice::from_raw_parts(value.as_ptr().cast(), len) };
        assert_eq!(Some(b'\0'), buf.last().copied());
        Self(&buf[..buf.len() - 1])
    }
}

impl Str<'_> {
    #[inline]
    pub fn as_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(self.0)
    }

    /// Converts to str without checking utf-8 validity.
    ///
    /// # Safety
    ///
    /// see [`core::str::from_utf8_unchecked`].
    #[inline]
    pub unsafe fn as_str_unchecked(&self) -> &str {
        str::from_utf8_unchecked(self.0)
    }
}

impl fmt::Display for Str<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        '"'.fmt(f)?;
        unsafe { self.as_str_unchecked() }.fmt(f)?;
        '"'.fmt(f)
    }
}

#[derive(Clone)]
pub struct StrList<'a>(&'a [u8]);

impl<'a> StrList<'a> {
    #[inline]
    pub(crate) fn new(value: &'a [StructureBlock], len: usize) -> Self {
        let buf = unsafe { slice::from_raw_parts(value.as_ptr().cast(), len) };
        assert_eq!(Some(b'\0'), buf.last().copied());
        Self(buf)
    }
}

impl<'a> Iterator for StrList<'a> {
    type Item = Str<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }
        let (head, tail) = self
            .0
            .split_at(slice::memchr::memchr(b'\0', self.0).unwrap());
        self.0 = &tail[1..];
        Some(Str(head))
    }
}

impl fmt::Display for StrList<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.clone();
        '['.fmt(f)?;
        if let Some(first) = iter.next() {
            first.fmt(f)?;
            for s in iter {
                ", ".fmt(f)?;
                s.fmt(f)?;
            }
        }
        ']'.fmt(f)
    }
}
