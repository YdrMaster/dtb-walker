use crate::{tree_on_stack::Node, Property, SkipType, Str, WalkOperation};
use core::fmt;

/// 遍历上下文。
pub struct Context<'a, T>(pub(crate) Node<'a, Inner<'a, T>>);

/// 遍历上下文的自定义部分。
pub trait ContextMeta: Sized {
    /// 遍历到一个子节点。
    ///
    /// - `_ctx`: 当前节点的上下文
    /// - `_name`: 子节点名
    #[inline]
    fn meet_child(&mut self, _ctx: &Context<Self>, _name: Str) -> WalkOperation<Self> {
        WalkOperation::Skip(SkipType::StepOver)
    }

    /// 遍历到一个属性。
    ///
    /// - `_ctx`: 当前节点的上下文
    /// - `_prop`: 属性对象
    #[inline]
    fn meet_prop(&mut self, _ctx: &Context<Self>, _prop: Property) -> SkipType {
        SkipType::StepOver
    }

    /// 从已退出的子节点收集信息。
    ///
    /// - `_child`: 子节点上下文元数据
    #[inline]
    fn collect_from_child(&mut self, _child: Self) -> SkipType {
        SkipType::StepOver
    }
}

pub(crate) struct Inner<'a, T> {
    pub name: Str<'a>,
    pub cells: Cells,
    pub meta: T,
}

impl<Meta> Context<'_, Meta> {
    pub(crate) const fn root(others: Meta) -> Self {
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

    /// 如果当前节点是根节点则返回 `true`。
    #[inline]
    pub fn is_root(&self) -> bool {
        self.0.is_root()
    }

    /// 返回当前节点名字。
    #[inline]
    pub fn name(&self) -> Str {
        self.0.as_ref().name
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
    pub(crate) fn grow(&'a self, name: Str<'a>, cells: Cells, meta: T) -> Self {
        Self(self.0.grow(Inner { name, cells, meta }))
    }
}

impl<Meta: ContextMeta> Context<'_, Meta> {
    #[allow(clippy::uninit_assumed_init)]
    pub(crate) fn meet_child(&mut self, name: Str) -> WalkOperation<Meta> {
        let mut meta = core::mem::replace(&mut self.0.data.meta, unsafe {
            core::mem::MaybeUninit::uninit().assume_init()
        });
        let ans = meta.meet_child(self, name);
        self.0.data.meta = meta;
        ans
    }

    #[allow(clippy::uninit_assumed_init)]
    pub(crate) fn meet_prop(&mut self, prop: Property) -> SkipType {
        let mut meta = core::mem::replace(&mut self.0.data.meta, unsafe {
            core::mem::MaybeUninit::uninit().assume_init()
        });
        let ans = meta.meet_prop(self, prop);
        self.0.data.meta = meta;
        ans
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
