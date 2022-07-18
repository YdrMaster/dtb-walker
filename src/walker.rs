use crate::{DtbObj, Path, Property, Reg, RegCfg, Str, StructureBlock as Blk, WalkOperation};
use core::slice;

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
        f: &mut impl FnMut(&Path<'_>, DtbObj) -> WalkOperation,
        path: &Path<'_>,
        reg_cfg: RegCfg,
        mut escape: bool,
    ) -> bool {
        use WalkOperation::*;

        let mut sub_reg_cfg = RegCfg::DEFAULT;
        loop {
            match self.tail.split_first() {
                // 子节点
                Some((&Blk::NODE_BEGIN, tail)) => {
                    // 找到字符串结尾
                    let name_len = tail.iter().position(Blk::is_end_of_str).unwrap() + 1;
                    let (name, tail) = tail.split_at(name_len);
                    self.tail = tail;
                    if escape {
                        // 如果当前子树已选跳过，不可能再选择终止
                        assert!(self.walk_inner(f, path, sub_reg_cfg, true));
                    } else {
                        // 正确舍弃尾 '\0'
                        let name = Str(unsafe {
                            slice::from_raw_parts(
                                name.as_ptr().cast::<u8>(),
                                name.len() * Blk::LEN - name.last().unwrap().str_tail_zero(),
                            )
                        });
                        let escape = match f(path, DtbObj::SubNode { name }) {
                            StepInto => false,
                            StepOver => true,
                            StepOut => {
                                escape = true;
                                true
                            }
                            Terminate => return false,
                        };
                        if !self.walk_inner(f, &path.grow(name), sub_reg_cfg, escape) {
                            return false;
                        }
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
                    if !escape {
                        let op = match self.prop_name(*nameoff) {
                            b"#address-cells" if value.len() == 1 => {
                                sub_reg_cfg.address_cells = value[0].into_u32();
                                StepOver
                            }
                            b"#size-cells" if value.len() == 1 => {
                                sub_reg_cfg.size_cells = value[0].into_u32();
                                StepOver
                            }
                            b"reg" if value.len() % reg_cfg.item_size() == 0 => f(
                                path,
                                DtbObj::Property(Property::Reg(Reg {
                                    buf: value,
                                    cfg: reg_cfg,
                                })),
                            ),
                            name => f(path, DtbObj::Property(Property::new(name, value, len))),
                        };
                        match op {
                            StepInto | StepOver => {}
                            StepOut => escape = true,
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
