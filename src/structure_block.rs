//! §5.4

use crate::U32BigEndian;

/// 结构块。
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct StructureBlock(U32BigEndian);

impl StructureBlock {
    /// 结构块的字节长度。
    pub const LEN: usize = core::mem::size_of::<Self>();

    /// 空字符串。
    pub const EMPTY_STR: Self = Self(U32BigEndian::from_u32(0));

    /// §5.4.1 FDT_BEGIN_NODE
    pub const NODE_BEGIN: Self = Self(U32BigEndian::from_u32(1));

    /// §5.4.1 FDT_END_NODE
    pub const NODE_END: Self = Self(U32BigEndian::from_u32(2));

    /// §5.4.1 FDT_PROP
    pub const PROP: Self = Self(U32BigEndian::from_u32(3));

    /// §5.4.1 FDT_NOP
    pub const NOP: Self = Self(U32BigEndian::from_u32(4));

    /// §5.4.1 FDT_END
    pub const END: Self = Self(U32BigEndian::from_u32(9));
}
