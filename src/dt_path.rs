use core::{fmt, ptr, str};

pub struct DtPath<'a> {
    pub(crate) parent: *const DtPath<'a>,
    pub(crate) name: &'a [u8],
}

impl DtPath<'_> {
    const ROOT: Self = Self {
        parent: ptr::null(),
        name: &[],
    };

    #[inline]
    pub(crate) fn root() -> *const Self {
        &Self::ROOT as _
    }

    /// 计算当前节点子树级别
    pub fn level(&self) -> usize {
        let mut ans = 0;
        let mut ptr = self.parent;
        while let Some(parent) = unsafe { ptr.as_ref() } {
            ans += 1;
            ptr = parent.parent;
        }
        ans
    }

    /// 将路径字符串格式化到 `buf` 中。
    ///
    /// 如果返回 `Ok(n)`，表示字符串长度为 `n`（`n` 不大于 `buf.len()`）。
    /// 如果返回 `Err(n)`，表示缓冲区长度无法存放整个字符串，实现保证 `n` 等于 `buf.len()`。
    pub fn join(&self, buf: &mut [u8]) -> Result<usize, usize> {
        let len = match unsafe { self.parent.as_ref() } {
            Some(parent) => parent.join(buf)?,
            None => return Ok(0),
        };
        match buf.len() - len {
            0 => Err(buf.len()),
            mut rest => {
                buf[len] = b'/';
                rest -= 1;
                if self.name.len() > rest {
                    buf[len + 1..].copy_from_slice(&self.name[..rest]);
                    Err(buf.len())
                } else {
                    buf[len + 1..][..self.name.len()].copy_from_slice(self.name);
                    Ok(len + self.name.len() + 1)
                }
            }
        }
    }
}

impl fmt::Display for DtPath<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(parent) = unsafe { self.parent.as_ref() } {
            parent.fmt(f)?;
            '/'.fmt(f)?;
            unsafe { core::str::from_utf8_unchecked(self.name) }.fmt(f)
        } else {
            Ok(())
        }
    }
}
