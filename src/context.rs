use crate::{tree_on_stack::Node, Property, SkipType, Str, WalkOperation};
use core::fmt;

/// 遍历上下文。
pub struct Context<'a, T>(Node<'a, Inner<'a, T>>);

pub trait ContextMeta: Sized {
    fn meet_child(&mut self, context: &Context<Self>, name: Str) -> WalkOperation;

    fn meet_prop(&mut self, context: &Context<Self>, prop: Property) -> SkipType;
}

struct Inner<'a, T> {
    name: Str<'a>,
    cells: Cells,
    meta: T,
}

impl<T> Context<'_, T> {
    pub(crate) const fn root(others: T) -> Self {
        Self(Node::root(Inner {
            name: Str(b""),
            cells: Cells::DEFAULT,
            meta: others,
        }))
    }

    /// 返回路径层数。定义根节点的子节点层数为 0。
    #[inline]
    pub fn level(&self) -> usize {
        self.0.level()
    }

    /// 如果这是根节点的路径则返回 `true`。
    #[inline]
    pub fn is_root(&self) -> bool {
        self.0.is_root()
    }

    /// 返回路径最后一级的节点名。
    #[inline]
    pub fn name(&self) -> Str {
        self.0.as_ref().name
    }

    /// 返回路径最后一级的节点名。
    #[inline]
    pub fn data(&mut self) -> &mut T {
        &mut self.0.data.meta
    }

    #[inline]
    pub(crate) fn cells(&self) -> Cells {
        self.0.as_ref().cells
    }

    /// 将路径字符串格式化到 `buf` 中。
    ///
    /// 如果返回 `Ok(n)`，表示字符串长度为 `n`（`n` 不大于 `buf.len()`）。
    /// 如果返回 `Err(n)`，表示缓冲区长度无法存放整个字符串，实现保证 `n` 等于 `buf.len()`。
    pub fn fmt_path(&self, buf: &mut [u8]) -> Result<usize, usize> {
        self.0.fold(0, |len, inner| match buf.len() - len {
            0 => Err(buf.len()),
            mut rest => {
                let bytes = inner.name.as_bytes();
                buf[len] = b'/';
                rest -= 1;
                if bytes.len() > rest {
                    buf[len + 1..].copy_from_slice(&bytes[..rest]);
                    Err(buf.len())
                } else {
                    buf[len + 1..][..bytes.len()].copy_from_slice(bytes);
                    Ok(len + bytes.len() + 1)
                }
            }
        })
    }
}

impl<'a, T> Context<'a, T> {
    #[inline]
    pub(crate) fn grow(&'a self, name: Str<'a>, cells: Cells, others: T) -> Self {
        Self(self.0.grow(Inner {
            name,
            cells,
            meta: others,
        }))
    }
}

impl<T> fmt::Display for Context<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fold((), |(), inner| {
            '/'.fmt(f)?;
            unsafe { inner.name.as_str_unchecked() }.fmt(f)
        })
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Cells {
    pub address: u32,
    pub size: u32,
    pub interrupt: u32,
}

impl Cells {
    pub const DEFAULT: Self = Self {
        address: 2,
        size: 1,
        interrupt: 1,
    };

    #[inline]
    pub fn reg_size(&self) -> usize {
        (self.address + self.size) as _
    }
}
