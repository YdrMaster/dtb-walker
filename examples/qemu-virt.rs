use dtb_walker::{utils::indent, Dtb, DtbObj, HeaderError, WalkOperation};

const DEVICE_TREE: &[u8] = include_bytes!("qemu-virt.dtb");
const INDENT_WIDTH: usize = 4;

fn main() {
    let mut aligned = vec![0usize; DEVICE_TREE.len() / core::mem::size_of::<usize>()];
    unsafe {
        aligned
            .as_mut_ptr()
            .copy_from_nonoverlapping(DEVICE_TREE.as_ptr() as _, aligned.len());
    }

    let dtb = match unsafe { Dtb::from_raw_parts(aligned.as_ptr() as _) } {
        Ok(ans) => ans,
        Err(HeaderError::LastCompVersion) => {
            // ignore!
            unsafe { Dtb::from_raw_parts_unchecked(aligned.as_ptr() as _) }
        }
        Err(e) => panic!("Verify dtb header failed: {e:?}"),
    };
    dtb.walk(|path, obj| match obj {
        DtbObj::SubNode { name } => {
            println!("{}{path}/{}", indent(path.level(), INDENT_WIDTH), unsafe {
                core::str::from_utf8_unchecked(name)
            });
            WalkOperation::StepInto
        }
        DtbObj::Property(prop) => {
            let indent = indent(path.level(), INDENT_WIDTH);
            println!("{indent}{prop:?}");
            WalkOperation::StepOver
        }
    });
    println!("ok");
}
