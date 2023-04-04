extern crate bumpalo;

fn main() {
    unsafe {
        let mut _local0 = bumpalo::Bump::with_capacity(10995706271387654244);
        bumpalo::Bump::iter_allocated_chunks_raw(&(_local0));
        bumpalo::Bump::allocated_bytes(&(_local0));
    }
}
