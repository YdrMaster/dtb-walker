const DEVICE_TREE: &[u8] = include_bytes!("qemu-virt.dtb");

fn main() -> Result<(), String> {
    use dtb_walker::{
        Context, ContextMeta, Dtb, HeaderError as E, Property, SkipType, SkipType::*, Str,
        WalkOperation as Op,
    };

    let mut aligned = vec![0usize; DEVICE_TREE.len() / core::mem::size_of::<usize>()];
    unsafe {
        aligned
            .as_mut_ptr()
            .copy_from_nonoverlapping(DEVICE_TREE.as_ptr() as _, aligned.len());
    }

    #[derive(PartialEq, Eq, Debug)]
    enum Meta {
        Root(usize),
        Cpus(usize),
    }

    impl ContextMeta for Meta {
        fn meet_child(&mut self, _context: &Context<Self>, name: Str) -> Op<Self> {
            match self {
                Meta::Root(_) => {
                    if name == Str::from("cpus") {
                        Op::Access(Meta::Cpus(0))
                    } else {
                        Op::Skip(SkipType::StepOver)
                    }
                }
                Meta::Cpus(n) => {
                    if name.starts_with("cpu@") {
                        *n += 1;
                    }
                    Op::Skip(SkipType::StepOver)
                }
            }
        }

        fn meet_prop(&mut self, _context: &Context<Self>, _prop: Property) -> SkipType {
            StepOver
        }

        fn escape(&mut self, sub: Self) -> SkipType {
            match self {
                Meta::Root(x) => match sub {
                    Meta::Cpus(n) => {
                        *x = n;
                        Terminate
                    }
                    Meta::Root(_) => unreachable!(),
                },
                Meta::Cpus(_) => StepOver,
            }
        }
    }

    let ans = unsafe {
        Dtb::from_raw_parts_filtered(aligned.as_ptr() as _, |e| {
            matches!(e, E::Misaligned(4) | E::LastCompVersion(16))
        })
    }
    .map_err(|e| format!("verify header failed: {e:?}"))?
    .walk(Meta::Root(0));
    assert_eq!(ans, Meta::Root(4));
    Ok(())
}
