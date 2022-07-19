/// 树节点
pub struct Node<'a, T> {
    pub data: T,
    pub parent: Option<&'a Self>,
}

impl<T> Node<'_, T> {
    /// 构造一个树根节点。
    #[inline]
    pub const fn root(data: T) -> Self {
        Node { data, parent: None }
    }

    /// 判断节点是不是根节点。
    #[inline]
    pub const fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    /// 返回树层数。
    #[inline]
    pub fn level(&self) -> usize {
        self.parent.map_or(0, |node| node.level() + 1)
    }

    /// 递归左折叠。
    #[inline]
    pub fn fold<A, E>(&self, init: A, mut f: impl FnMut(A, &T) -> Result<A, E>) -> Result<A, E> {
        self.fold_inner(init, &mut f)
    }

    /// 递归右折叠。
    #[allow(unused)]
    #[inline]
    pub fn fold_r<A, E>(&self, init: A, mut f: impl FnMut(A, &T) -> Result<A, E>) -> Result<A, E> {
        self.fold_r_inner(init, &mut f)
    }

    fn fold_inner<A, E>(&self, init: A, f: &mut impl FnMut(A, &T) -> Result<A, E>) -> Result<A, E> {
        match self.parent {
            Some(parent) => {
                let a = parent.fold_inner(init, f)?;
                (*f)(a, &self.data)
            }
            None => Ok(init),
        }
    }

    fn fold_r_inner<A, E>(
        &self,
        init: A,
        f: &mut impl FnMut(A, &T) -> Result<A, E>,
    ) -> Result<A, E> {
        let a = (*f)(init, &self.data)?;
        match self.parent {
            Some(parent) => parent.fold_r_inner(a, f),
            None => Ok(a),
        }
    }
}

impl<T> AsRef<T> for Node<'_, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<'a, T> Node<'a, T> {
    /// 为树增加一个子节点。
    #[inline]
    pub fn grow(&'a self, data: T) -> Self {
        Node {
            data,
            parent: Some(self),
        }
    }
}
