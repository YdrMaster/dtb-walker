#![no_std]
#![feature(slice_internals)]

use core::{fmt, ptr, slice};

mod dt_path;
mod header;
mod reg;
mod structure_block;

pub use dt_path::DtPath;
pub use reg::Reg;

use header::{FdtHeader, HeaderError};
use reg::RegCfg;
use structure_block::StructureBlock;

/// 设备树递归结构。
pub struct DtbWalker<'a> {
    tail: &'a [StructureBlock],
    header: &'a FdtHeader,
    strings: &'a [u8],
}

impl DtbWalker<'static> {
    /// 构造设备树二进制对象递归遍历上下文。
    ///
    /// # Safety
    ///
    /// 如果指针指向一个有效的 DTB 首部，其中描述的各个数据段会被切片。
    pub unsafe fn new(ptr: *const u8) -> Result<Self, HeaderError> {
        let header: &FdtHeader = &*ptr.cast();
        header.verify()?;
        Ok(Self {
            tail: slice::from_raw_parts(
                ptr.offset(header.off_dt_struct.into_u32() as _)
                    .cast::<StructureBlock>()
                    .offset(2),
                (header.size_dt_struct.into_u32() as usize) / StructureBlock::LEN - 3,
            ),
            header,
            strings: slice::from_raw_parts(
                ptr.offset(header.off_dt_strings.into_u32() as _),
                header.size_dt_strings.into_u32() as _,
            ),
        })
    }
}

pub enum DtbObj<'a> {
    /// 一般属性
    Property { name: &'a [u8], value: &'a [u8] },
    /// 寄存器属性
    Reg(Reg<'a>),
    /// 子节点
    SubNode { name: &'a [u8] },
}

pub enum WalkOperation {
    /// 进入子节点
    StepInto,
    /// 跳过子节点
    StepOver,
    /// 跳过当前子树
    StepOut,
    /// 结束遍历
    Terminate,
}

impl<'a> DtbWalker<'a> {
    /// 遍历。
    pub fn walk(mut self, f: &mut impl FnMut(&DtPath<'_>, DtbObj) -> WalkOperation) {
        self.walk_inner(f, DtPath::root(), RegCfg::DEFAULT, false);
    }
}

impl DtbWalker<'_> {
    /// 切分属性名。
    fn prop_name(&self, nameoff: StructureBlock) -> &[u8] {
        let nameoff = nameoff.into_u32() as usize;
        let name = &self.strings[nameoff..];
        &name[..slice::memchr::memchr(b'\0', name).unwrap()]
    }

    /// 深度优先遍历。如果返回 `false`，取消所有后续的遍历。
    fn walk_inner(
        &mut self,
        f: &mut impl FnMut(&DtPath<'_>, DtbObj) -> WalkOperation,
        path: *const DtPath<'_>,
        reg_cfg: RegCfg,
        mut escape: bool,
    ) -> bool {
        use StructureBlock as Blk;
        use WalkOperation::*;

        let mut sub_reg_cfg = RegCfg::DEFAULT;
        loop {
            match self.tail.split_first() {
                Some((&Blk::NODE_BEGIN, tail)) => {
                    // 找到字符串结尾
                    let name_len = tail.iter().position(Blk::is_end_of_str).unwrap() + 1;
                    let (name, tail) = tail.split_at(name_len);
                    self.tail = tail;
                    if escape {
                        // 如果当前子树已选跳过，不可能再选择终止
                        assert!(self.walk_inner(f, ptr::null(), sub_reg_cfg, true));
                    } else {
                        // 正确舍弃尾 '\0'
                        let name = unsafe {
                            slice::from_raw_parts(
                                name.as_ptr().cast::<u8>(),
                                name.len() * Blk::LEN - name.last().unwrap().str_tail_zero(),
                            )
                        };
                        let escape = match f(unsafe { &*path }, DtbObj::SubNode { name }) {
                            StepInto => false,
                            StepOver => true,
                            StepOut => {
                                escape = true;
                                true
                            }
                            Terminate => return false,
                        };
                        let path = DtPath { parent: path, name };
                        if !self.walk_inner(f, &path as _, sub_reg_cfg, escape) {
                            return false;
                        }
                    }
                }
                Some((&Blk::NODE_END, tail)) => {
                    self.tail = tail;
                    return true;
                }
                Some((&Blk::PROP, [len, nameoff, tail @ ..])) => {
                    // 切分属性值
                    let len = len.into_u32() as usize;
                    let (value, tail) = tail.split_at((len + Blk::LEN - 1) / Blk::LEN);
                    // 如果当前子树需要解析
                    if !escape {
                        let op = match self.prop_name(*nameoff) {
                            b"#address-cells" => match *value {
                                [blk] => {
                                    sub_reg_cfg.address_cells = blk.into_u32();
                                    StepOver
                                }
                                _ => panic!(),
                            },
                            b"#size-cells" => match *value {
                                [blk] => {
                                    sub_reg_cfg.size_cells = blk.into_u32();
                                    StepOver
                                }
                                _ => panic!(),
                            },
                            b"reg" => f(unsafe { &*path }, DtbObj::Reg(Reg::new(value, reg_cfg))),
                            name => f(
                                unsafe { &*path },
                                DtbObj::Property {
                                    name,
                                    value: unsafe {
                                        slice::from_raw_parts(value.as_ptr().cast(), len)
                                    },
                                },
                            ),
                        };
                        match op {
                            StepInto | StepOver => {}
                            StepOut => escape = true,
                            Terminate => return false,
                        };
                    }
                    self.tail = tail;
                }
                Some((&Blk::NOP, tail)) => self.tail = tail,
                Some((token, _)) => unreachable!("{token:#x?}"),
                None => unreachable!(),
            }
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
struct U32BigEndian(u32);

impl U32BigEndian {
    #[inline]
    pub const fn from_u32(val: u32) -> Self {
        Self(u32::to_be(val))
    }

    #[inline]
    pub const fn into_u32(self) -> u32 {
        u32::from_be(self.0)
    }
}

impl fmt::Debug for U32BigEndian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        u32::from_be(self.0).fmt(f)
    }
}

#[inline]
fn is_aligned(val: usize, bits: usize) -> bool {
    val & (bits - 1) == 0
}
