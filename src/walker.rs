use crate::{
    DtbObj, PHandle, Path, Reg, RegCfg, Str, StrList, StructureBlock as Blk, WalkOperation,
};
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
        &name[..slice::memchr::memchr(b'\0', name).unwrap()]
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
                        let name = unsafe {
                            slice::from_raw_parts(
                                name.as_ptr().cast::<u8>(),
                                name.len() * Blk::LEN - name.last().unwrap().str_tail_zero(),
                            )
                        };
                        let escape = match f(path, DtbObj::SubNode { name }) {
                            StepInto => false,
                            StepOver => true,
                            StepOut => {
                                escape = true;
                                true
                            }
                            Terminate => return false,
                        };
                        if !self.walk_inner(
                            f,
                            &Path {
                                parent: Some(path),
                                name,
                            },
                            sub_reg_cfg,
                            escape,
                        ) {
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
                            b"compatible" => f(path, DtbObj::Compatible(StrList::new(value, len))),
                            b"model" => f(path, DtbObj::Model(Str::new(value, len))),
                            b"reg" => f(path, DtbObj::Reg(Reg::new(value, reg_cfg))),
                            b"phandle" | b"linux,phandle" => {
                                f(path, DtbObj::PHandle(PHandle::new(value)))
                            }
                            name => f(
                                path,
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
                // 跳过
                Some((&Blk::NOP, tail)) => self.tail = tail,
                Some((_, _)) | None => unreachable!(),
            }
        }
    }
}
