//! §5.4

use crate::U32BigEndian;

/// 结构块。
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

    /// Converts to `u32`.
    pub const fn into_u32(self) -> u32 {
        self.0.into_u32()
    }

    /// 一个 '\0' 结尾字符串结束于此块。
    pub const fn is_end_of_str(&self) -> bool {
        matches!(self.0 .0.to_ne_bytes(), [_, _, _, 0])
    }

    /// 字符串结尾 '\0' 数量。
    pub const fn str_tail_zero(&self) -> usize {
        match self.0 .0.to_ne_bytes() {
            [0, _, _, _] => 4,
            [_, 0, _, _] => 3,
            [_, _, 0, _] => 2,
            [_, _, _, _] => 1,
        }
    }
}
