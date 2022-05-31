use dtb_walker::{utils::indent, Dtb, DtbObj, Property, WalkOperation};

const DEVICE_TREE: &[u8] = include_bytes!("qemu-virt.dtb");
const INDENT_WIDTH: usize = 4;

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
            println!("{}{path}/{}", indent(path.level(), INDENT_WIDTH), unsafe {
                core::str::from_utf8_unchecked(name)
            });
            WalkOperation::StepInto
        }
        DtbObj::Property(prop) => {
            let indent = indent(path.level(), INDENT_WIDTH);
            match prop {
                Property::Compatible(compatible) => println!("{indent}compatible = {compatible};"),
                Property::Model(model) => println!("{indent}model = {model};"),
                Property::Reg(reg) => println!("{indent}reg = {reg:#x?};"),
                Property::PHandle(phandle) => println!("{indent}phandle = {phandle:?};"),
                Property::General { name, value } => {
                    print!("{indent}{}", unsafe {
                        core::str::from_utf8_unchecked(name)
                    });
                    match name {
                        _ if !value.is_empty() => {
                            println!(" = {value:02x?};");
                        }
                        _ => {
                            println!(";");
                        }
                    }
                }
            }
            WalkOperation::StepOver
        }
    });
    println!("ok");
}
