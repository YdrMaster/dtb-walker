use crate::Str;
use core::{fmt, str};

/// 设备树节点路径。
pub struct Path<'a> {
    pub(crate) parent: Option<&'a Path<'a>>,
    pub(crate) name: Str<'a>,
}

impl Path<'_> {
    pub(crate) const ROOT: Self = Self {
        parent: None,
        name: Str(&[]),
    };

    /// 返回路径层数。定义根节点的子节点层数为 0。
    #[inline]
    pub fn level(&self) -> usize {
        if let Some(parent) = self.parent {
            parent.level() + 1
        } else {
            0
        }
    }

    /// 返回路径最后一级的节点名。
    #[inline]
    pub fn last(&self) -> Str {
        self.name
    }

    /// 如果这是根节点的路径则返回 `true`。
    #[inline]
    pub fn is_root(&self) -> bool {
        self.name.0.is_empty()
    }

    /// 将路径字符串格式化到 `buf` 中。
    ///
    /// 如果返回 `Ok(n)`，表示字符串长度为 `n`（`n` 不大于 `buf.len()`）。
    /// 如果返回 `Err(n)`，表示缓冲区长度无法存放整个字符串，实现保证 `n` 等于 `buf.len()`。
    pub fn join(&self, buf: &mut [u8]) -> Result<usize, usize> {
        let len = match self.parent {
            Some(parent) => parent.join(buf)?,
            None => return Ok(0),
        };
        match buf.len() - len {
            0 => Err(buf.len()),
            mut rest => {
                buf[len] = b'/';
                rest -= 1;
                if self.name.0.len() > rest {
                    buf[len + 1..].copy_from_slice(&self.name.0[..rest]);
                    Err(buf.len())
                } else {
                    buf[len + 1..][..self.name.0.len()].copy_from_slice(self.name.0);
                    Ok(len + self.name.0.len() + 1)
                }
            }
        }
    }
}

impl fmt::Display for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(parent) = self.parent {
            parent.fmt(f)?;
            '/'.fmt(f)?;
            unsafe { str::from_utf8_unchecked(self.name.0) }.fmt(f)
        } else {
            Ok(())
        }
    }
}
