use dtb_walker::{utils::indent, Dtb, DtbObj, WalkOperation};

const DEVICE_TREE: &[u8] = include_bytes!("qemu-virt.dtb");

fn main() {
    let mut aligned = vec![0usize; DEVICE_TREE.len() / core::mem::size_of::<usize>()];
    unsafe {
        aligned
            .as_mut_ptr()
            .copy_from_nonoverlapping(DEVICE_TREE.as_ptr() as _, aligned.len());
    }

    let dtb = unsafe { Dtb::from_raw_parts(aligned.as_ptr() as _) }.unwrap();
    dtb.walk(|path, obj| match obj {
        DtbObj::SubNode { name } => {
            println!("{}{path}/{}", indent(path.level(), 2), unsafe {
                core::str::from_utf8_unchecked(name)
            });
            WalkOperation::StepInto
        }
        DtbObj::Property { name, value } => {
            print!("{}prop {}", indent(path.level(), 2), unsafe {
                core::str::from_utf8_unchecked(name)
            });
            match name {
                b"compatible" | b"model" => {
                    println!(": {}", unsafe { core::str::from_utf8_unchecked(value) });
                }
                _ if !value.is_empty() => {
                    println!(": {value:02x?}");
                }
                _ => {
                    println!();
                }
            }
            WalkOperation::StepInto
        }
        DtbObj::Reg(reg) => {
            println!("{}prop reg:", indent(path.level(), 2));
            for reg in reg {
                println!("{}{reg:#x?}", indent(path.level() + 1, 2));
            }
            WalkOperation::StepInto
        }
    });
    println!("ok");
}
