use dtb_walker::DtbWalker;

const DEVICE_TREE: &[u8] = include_bytes!("qemu-virt.dtb");

fn main() {
    let mut aligned = vec![0usize; DEVICE_TREE.len() / core::mem::size_of::<usize>()];
    unsafe {
        aligned
            .as_mut_ptr()
            .copy_from_nonoverlapping(DEVICE_TREE.as_ptr() as _, aligned.len());
    }
    let walker = unsafe { DtbWalker::new(aligned.as_ptr() as _) }.unwrap();
    println!("ok");
}
