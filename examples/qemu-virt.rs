﻿use dtb_walker::{DtbObj, DtbWalker, WalkOperation};

const DEVICE_TREE: &[u8] = include_bytes!("qemu-virt.dtb");

fn main() {
    let mut aligned = vec![0usize; DEVICE_TREE.len() / core::mem::size_of::<usize>()];
    unsafe {
        aligned
            .as_mut_ptr()
            .copy_from_nonoverlapping(DEVICE_TREE.as_ptr() as _, aligned.len());
    }
    let walker = unsafe { DtbWalker::new(aligned.as_ptr() as _) }.unwrap();
    walker.walk(&mut |path, obj| match obj {
        DtbObj::Property { name, value } => {
            println!(
                "{}prop {}: {}",
                " ".repeat(path.level() * 2),
                unsafe { core::str::from_utf8_unchecked(name) },
                unsafe { core::str::from_utf8_unchecked(value) },
            );
            WalkOperation::StepInto
        }
        DtbObj::Reg(_) => {
            println!("{}prop reg", " ".repeat(path.level() * 2));
            WalkOperation::StepInto
        }
        DtbObj::SubNode { name } => {
            println!("{}subnode {}", " ".repeat(path.level() * 2), unsafe {
                core::str::from_utf8_unchecked(name)
            });
            WalkOperation::StepInto
        }
    });
    println!("ok");
}
