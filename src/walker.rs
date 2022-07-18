use crate::{
    context::Cells, Context, DtbObj, Property, Reg, RegCfg, SkipType, Str, StructureBlock as Blk,
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
    pub fn walk_inner<T: Default>(
        &mut self,
        f: &mut impl FnMut(&Context<'_, T>, DtbObj) -> WalkOperation,
        ctx: Context<'_, T>,
    ) -> bool {
        use SkipType::*;
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
                    // 正确舍弃尾 '\0'
                    let name = Str(unsafe {
                        core::slice::from_raw_parts(
                            name.as_ptr().cast::<u8>(),
                            name.len() * Blk::LEN - name.last().unwrap().str_tail_zero(),
                        )
                    });
                    match f(&ctx, DtbObj::SubNode { name }) {
                        Access => {
                            let ctx = ctx.grow(name, cells, T::default());
                            if !self.walk_inner(f, ctx) {
                                return false;
                            }
                        }
                        Skip(ty) => match ty {
                            StepOver => self.skip_inner(),
                            StepOut => {
                                self.skip_inner();
                                return true;
                            }
                            Terminate => return false,
                        },
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
                    self.tail = tail;
                    // 如果当前子树需要解析
                    let ty = match self.prop_name(*nameoff) {
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
                        b"reg" if value.len() % (ctx.cells().reg_size()) == 0 => match f(
                            &ctx,
                            DtbObj::Property(Property::Reg(Reg {
                                buf: value,
                                cfg: RegCfg {
                                    address_cells: ctx.cells().address,
                                    size_cells: ctx.cells().size,
                                },
                            })),
                        ) {
                            Access => unreachable!(),
                            Skip(ty) => ty,
                        },
                        name => match f(&ctx, DtbObj::Property(Property::new(name, value, len))) {
                            Access => unreachable!(),
                            Skip(ty) => ty,
                        },
                    };
                    match ty {
                        StepOver => {}
                        StepOut => {
                            self.skip_inner();
                            return true;
                        }
                        Terminate => return false,
                    };
                }
                // 跳过
                Some((&Blk::NOP, tail)) => self.tail = tail,
                Some((_, _)) | None => unreachable!(),
            }
        }
    }

    /// 跳过节点
    fn skip_inner(&mut self) {
        loop {
            match self.tail.split_first() {
                // 子节点
                Some((&Blk::NODE_BEGIN, tail)) => {
                    // 找到字符串结尾
                    let name_len = tail.iter().position(Blk::is_end_of_str).unwrap() + 1;
                    self.tail = &tail[name_len..];
                    self.skip_inner();
                }
                // 当前节点结束
                Some((&Blk::NODE_END, tail)) => {
                    self.tail = tail;
                    break;
                }
                // 属性
                Some((&Blk::PROP, [len, _nameoff, tail @ ..])) => {
                    // 切分属性值
                    self.tail = &tail[(((len.into_u32() as usize) + Blk::LEN - 1) / Blk::LEN)..];
                }
                // 跳过
                Some((&Blk::NOP, tail)) => self.tail = tail,
                Some((_, _)) | None => unreachable!(),
            }
        }
    }
}
