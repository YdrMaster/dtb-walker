use crate::{tree_on_stack::Node, Str};
use core::fmt;

/// 设备树节点路径。
pub struct Path<'a>(Node<'a, Str<'a>>);

impl Path<'_> {
    pub(crate) const ROOT: Self = Self(Node::root(Str(&[])));

    /// 返回路径层数。定义根节点的子节点层数为 0。
    #[inline]
    pub fn level(&self) -> usize {
        self.0.level()
    }

    /// 返回路径最后一级的节点名。
    #[inline]
    pub fn last(&self) -> Str {
        *self.0.as_ref()
    }

    /// 如果这是根节点的路径则返回 `true`。
    #[inline]
    pub fn is_root(&self) -> bool {
        self.0.is_root()
    }

    /// 将路径字符串格式化到 `buf` 中。
    ///
    /// 如果返回 `Ok(n)`，表示字符串长度为 `n`（`n` 不大于 `buf.len()`）。
    /// 如果返回 `Err(n)`，表示缓冲区长度无法存放整个字符串，实现保证 `n` 等于 `buf.len()`。
    pub fn join(&self, buf: &mut [u8]) -> Result<usize, usize> {
        self.0.fold(0, |len, name| match buf.len() - len {
            0 => Err(buf.len()),
            mut rest => {
                buf[len] = b'/';
                rest -= 1;
                if name.0.len() > rest {
                    buf[len + 1..].copy_from_slice(&name.0[..rest]);
                    Err(buf.len())
                } else {
                    buf[len + 1..][..name.0.len()].copy_from_slice(name.0);
                    Ok(len + name.0.len() + 1)
                }
            }
        })
    }
}

impl<'a> Path<'a> {
    pub(crate) fn grow(&'a self, node: Str<'a>) -> Self {
        Self(self.0.grow(node))
    }
}

impl fmt::Display for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fold((), |(), name| {
            '/'.fmt(f)?;
            unsafe { name.as_str_unchecked() }.fmt(f)
        })
    }
}
