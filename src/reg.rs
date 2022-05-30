use crate::StructureBlock;
use core::ops::Range;

/// `reg` 属性。
#[derive(Clone)]
pub struct Reg<'a> {
    buf: &'a [StructureBlock],
    cfg: RegCfg,
}

impl<'a> Reg<'a> {
    /// 检查属性值切片长度符合父节点设定的 `reg` 格式。
    #[inline]
    pub(crate) fn new(buf: &'a [StructureBlock], cfg: RegCfg) -> Self {
        assert_eq!(0, buf.len() % (cfg.address_cells + cfg.size_cells) as usize);
        Self { buf, cfg }
    }
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

#[derive(Clone, Copy)]
pub(crate) struct RegCfg {
    pub address_cells: u32,
    pub size_cells: u32,
}

impl RegCfg {
    pub const DEFAULT: Self = Self {
        address_cells: 2,
        size_cells: 1,
    };
}
