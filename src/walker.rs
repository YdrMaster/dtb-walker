use crate::{
    context::{Cells, ContextMeta},
    Context, Property, Reg, RegCfg, SkipType, Str, StructureBlock as Blk, WalkOperation as Op,
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
    pub fn walk_inner<T: ContextMeta>(&mut self, ctx: &mut Context<'_, T>) -> bool {
        use SkipType::*;

        loop {
            match self.tail.split_first() {
                // 子节点
                Some((&Blk::NODE_BEGIN, tail)) => {
                    // 找到字符串结尾
                    let name_len = tail.iter().position(Blk::is_end_of_str).unwrap() + 1;
                    let (name, tail) = tail.split_at(name_len);
                    self.tail = tail;
                    // 舍弃尾 '\0' 构造字符串
                    let name = Str(unsafe {
                        core::slice::from_raw_parts(
                            name.as_ptr().cast::<u8>(),
                            name.len() * Blk::LEN - name.last().unwrap().str_tail_zero(),
                        )
                    });
                    match ctx.meet_child(name) {
                        Op::Access(meta) => {
                            let mut sub = ctx.grow(name, Cells::DEFAULT, meta);
                            let ans = self.walk_inner(&mut sub);
                            ctx.0.data.meta.escape(sub.0.data.meta);
                            if !ans {
                                return false;
                            }
                        }
                        Op::Skip(ty) => match ty {
                            StepOver => {
                                // 跳出子节点
                                self.skip_inner();
                            }
                            StepOut => {
                                // 跳出子节点
                                self.skip_inner();
                                // 跳出当前节点
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
                    let prop = match self.prop_name(*nameoff) {
                        b"#address-cells" if value.len() == 1 => {
                            ctx.0.data.cells.address = value[0].into_u32();
                            continue;
                        }
                        b"#size-cells" if value.len() == 1 => {
                            ctx.0.data.cells.size = value[0].into_u32();
                            continue;
                        }
                        b"#interrupt-cells" if value.len() == 1 => {
                            ctx.0.data.cells.interrupt = value[0].into_u32();
                            continue;
                        }
                        b"reg" => {
                            // 如果遇到 reg 属性，必定有父节点提供解析方法
                            let cells = ctx.0.parent.unwrap().data.cells;
                            if value.len() % (cells.reg_size()) == 0 {
                                Property::Reg(Reg {
                                    buf: value,
                                    cfg: RegCfg {
                                        address_cells: cells.address,
                                        size_cells: cells.size,
                                    },
                                })
                            } else {
                                Property::new(b"reg", value, len)
                            }
                        }
                        name => Property::new(name, value, len),
                    };
                    match ctx.meet_prop(prop) {
                        StepOver => {}
                        StepOut => {
                            // 跳出当前节点
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
