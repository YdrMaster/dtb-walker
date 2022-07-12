const DEVICE_TREE: &[u8] = include_bytes!("qemu-virt.dtb");
const INDENT_WIDTH: usize = 4;

fn main() -> Result<(), String> {
    use dtb_walker::{utils::indent, Dtb, DtbObj, HeaderError as E, WalkOperation as Op};

    let mut aligned = vec![0usize; DEVICE_TREE.len() / core::mem::size_of::<usize>()];
    unsafe {
        aligned
            .as_mut_ptr()
            .copy_from_nonoverlapping(DEVICE_TREE.as_ptr() as _, aligned.len());
    }

    let dtb = unsafe {
        Dtb::from_raw_parts_filtered(aligned.as_ptr() as _, |e| {
            matches!(e, E::Misaligned(4) | E::LastCompVersion(16))
        })
    }
    .map_err(|e| format!("verify header failed: {e:?}"))?;
    dtb.walk(|path, obj| match obj {
        DtbObj::SubNode { name } => {
            println!("{}{path}/{name}", indent(path.level(), INDENT_WIDTH));
            Op::StepInto
        }
        DtbObj::Property(prop) => {
            let indent = indent(path.level(), INDENT_WIDTH);
            println!("{indent}{prop:?}");
            Op::StepOver
        }
    });
    Ok(())
}
