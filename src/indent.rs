use core::fmt;

pub struct Indent {
    level: usize,
    width: usize,
}

/// 构造一个 `level` 级，每级宽 `width` 的缩进。
#[inline]
pub const fn indent(level: usize, width: usize) -> Indent {
    Indent { level, width }
}

impl fmt::Display for Indent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..self.level {
            for _ in 0..self.width {
                ' '.fmt(f)?;
            }
        }
        Ok(())
    }
}
