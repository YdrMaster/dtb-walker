//! see §2.2.4/Property-Values

use super::{Error, Result as Res};
use crate::{Str, StructureBlock};
use core::{fmt, slice};

impl<'a> Str<'a> {
    #[inline]
    pub(super) fn new(value: &'a [StructureBlock], len: usize) -> Res<Self> {
        let buf = unsafe { slice::from_raw_parts(value.as_ptr().cast(), len) };
        if let Some(b'\0') = buf.last().copied() {
            Ok(Self(&buf[..buf.len() - 1]))
        } else {
            Err(Error)
        }
    }
}

/// `<stringlist>` 类型的属性值。
#[derive(Clone)]
pub struct StrList<'a>(&'a [u8]);

impl<'a> StrList<'a> {
    #[inline]
    pub(super) fn new(value: &'a [StructureBlock], len: usize) -> Res<Self> {
        let buf = unsafe { slice::from_raw_parts(value.as_ptr().cast(), len) };
        if let Some(b'\0') = buf.last().copied() {
            Ok(Self(buf))
        } else {
            Err(Error)
        }
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
            .split_at(self.0.iter().position(|c| *c == b'\0').unwrap());
        self.0 = &tail[1..];
        Some(Str(head))
    }
}

impl fmt::Display for StrList<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.clone();
        '['.fmt(f)?;
        if let Some(first) = iter.next() {
            '"'.fmt(f)?;
            first.fmt(f)?;
            '"'.fmt(f)?;
            for s in iter {
                ", ".fmt(f)?;
                '"'.fmt(f)?;
                s.fmt(f)?;
                '"'.fmt(f)?;
            }
        }
        ']'.fmt(f)
    }
}
