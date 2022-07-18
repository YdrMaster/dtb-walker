use crate::StructureBlock;
use core::{fmt, ops::Range};

/// `reg` 属性。
#[derive(Clone)]
pub struct Reg<'a> {
    pub(crate) buf: &'a [StructureBlock],
    pub(crate) cfg: RegCfg,
}

impl Iterator for Reg<'_> {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            return None;
        }
        let RegCfg {
            address_cells,
            size_cells,
        } = self.cfg;
        let (base_buf, tail) = self.buf.split_at(address_cells as _);
        let (size_buf, tail) = tail.split_at(size_cells as _);
        self.buf = tail;

        let base = base_buf
            .iter()
            .fold(0usize, |acc, it| (acc << 32) + it.into_u32() as usize);
        let size = size_buf
            .iter()
            .fold(0usize, |acc, it| (acc << 32) + it.into_u32() as usize);
        Some(base..base + size)
    }
}

impl fmt::Debug for Reg<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.clone();
        write!(f, "[")?;
        if let Some(first) = iter.next() {
            first.fmt(f)?;
            for s in iter {
                write!(f, ", ")?;
                s.fmt(f)?;
            }
        }
        write!(f, "]")
    }
}

#[derive(Clone, Copy)]
pub(crate) struct RegCfg {
    pub address_cells: u32,
    pub size_cells: u32,
}
