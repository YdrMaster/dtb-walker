pub struct DtPath<'a> {
    pub(crate) parent: *const DtPath<'a>,
    pub(crate) name: &'a [u8],
}

impl DtPath<'_> {
    pub const ROOT: Self = Self {
        parent: core::ptr::null(),
        name: &[],
    };

    pub fn level(&self) -> usize {
        let mut ans = 0;
        let mut ptr = self.parent;
        while let Some(parent) = unsafe { ptr.as_ref() } {
            ans += 1;
            ptr = parent.parent;
        }
        ans
    }
}
