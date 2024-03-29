﻿use crate::{
    context::Cells, Context, DtbObj, Property, Reg, RegCfg, Str, StructureBlock as Blk,
    WalkOperation,
};

/// 设备树递归结构。
pub(crate) struct Walker<'a> {
    pub tail: &'a [Blk],
    pub strings: &'a [u8],
}

impl Walker<'_> {
    /// 切分属性名。
    fn prop_name(&self, nameoff: Blk) -> &[u8] {
        let nameoff = nameoff.into_u32() as usize;
        let name = &self.strings[nameoff..];
        &name[..name.iter().position(|c| *c == b'\0').unwrap()]
    }

    /// 深度优先遍历。如果返回 `false`，取消所有后续的遍历。
    pub fn walk_inner(
        &mut self,
        f: &mut impl FnMut(&Context<'_>, DtbObj) -> WalkOperation,
        mut ctx: Option<Context>,
    ) -> bool {
        use WalkOperation::*;

        let mut cells = Cells::DEFAULT;
        loop {
            match self.tail.split_first() {
                // 子节点
                Some((&Blk::NODE_BEGIN, tail)) => {
                    // 找到字符串结尾
                    let name_len = tail.iter().position(Blk::is_end_of_str).unwrap() + 1;
                    let (name, tail) = tail.split_at(name_len);
                    self.tail = tail;
                    if let Some(ctx_) = ctx.as_ref() {
                        // 正确舍弃尾 '\0'
                        let name = Str(unsafe {
                            core::slice::from_raw_parts(
                                name.as_ptr().cast::<u8>(),
                                name.len() * Blk::LEN - name.last().unwrap().str_tail_zero(),
                            )
                        });
                        let ctx = match f(ctx_, DtbObj::SubNode { name }) {
                            StepInto => Some(ctx_.grow(name, cells)),
                            StepOver => None,
                            StepOut => {
                                ctx = None;
                                None
                            }
                            Terminate => return false,
                        };
                        if !self.walk_inner(f, ctx) {
                            return false;
                        }
                    } else {
                        // 如果当前子树已选跳过，不可能再选择终止
                        assert!(self.walk_inner(f, None));
                    }
                }
                // 当前节点结束
                Some((&Blk::NODE_END, tail)) => {
                    self.tail = tail;
                    return true;
                }
                // 属性
                Some((&Blk::PROP, [len, nameoff, tail @ ..])) => {
                    // 切分属性值
                    let len = len.into_u32() as usize;
                    let (value, tail) = tail.split_at((len + Blk::LEN - 1) / Blk::LEN);
                    // 如果当前子树需要解析
                    if let Some(ctx_) = ctx.as_ref() {
                        let op = match self.prop_name(*nameoff) {
                            b"#address-cells" if value.len() == 1 => {
                                cells.address = value[0].into_u32();
                                StepOver
                            }
                            b"#size-cells" if value.len() == 1 => {
                                cells.size = value[0].into_u32();
                                StepOver
                            }
                            b"#interrupt-cells" if value.len() == 1 => {
                                cells.interrupt = value[0].into_u32();
                                StepOver
                            }
                            b"reg" if value.len() % (ctx_.cells().reg_size()) == 0 => f(
                                ctx_,
                                DtbObj::Property(Property::Reg(Reg {
                                    buf: value,
                                    cfg: RegCfg {
                                        address_cells: ctx_.cells().address,
                                        size_cells: ctx_.cells().size,
                                    },
                                })),
                            ),
                            name => f(ctx_, DtbObj::Property(Property::new(name, value, len))),
                        };
                        match op {
                            StepInto | StepOver => {}
                            StepOut => ctx = None,
                            Terminate => return false,
                        };
                    }
                    self.tail = tail;
                }
                // 跳过
                Some((&Blk::NOP, tail)) => self.tail = tail,
                Some((_, _)) | None => unreachable!(),
            }
        }
    }
}
