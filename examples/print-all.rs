const DEVICE_TREE: &[u8] = include_bytes!("qemu-virt.dtb");
const INDENT_WIDTH: usize = 4;

fn main() -> Result<(), String> {
    use dtb_walker::{
        utils::indent, ContextMeta, Dtb, HeaderError as E, SkipType::*, WalkOperation as Op,
    };

    let mut aligned = vec![0usize; DEVICE_TREE.len() / core::mem::size_of::<usize>()];
    unsafe {
        aligned
            .as_mut_ptr()
            .copy_from_nonoverlapping(DEVICE_TREE.as_ptr() as _, aligned.len());
    }

    struct Meta;

    impl ContextMeta for Meta {
        fn meet_child(
            &mut self,
            context: &dtb_walker::Context<Self>,
            name: dtb_walker::Str,
        ) -> Op<Self> {
            println!("{}{context}/{name}", indent(context.level(), INDENT_WIDTH));
            Op::Access(Self)
        }

        fn meet_prop(
            &mut self,
            context: &dtb_walker::Context<Self>,
            prop: dtb_walker::Property,
        ) -> dtb_walker::SkipType {
            println!("{}{prop:?}", indent(context.level(), INDENT_WIDTH));
            StepOver
        }

        fn escape(&mut self, _sub: Self) -> dtb_walker::SkipType {
            StepOver
        }
    }

    unsafe {
        Dtb::from_raw_parts_filtered(aligned.as_ptr() as _, |e| {
            matches!(e, E::Misaligned(4) | E::LastCompVersion(16))
        })
    }
    .map_err(|e| format!("verify header failed: {e:?}"))?
    .walk(Meta);
    Ok(())
}
