extern crate bumpalo;

fn main() {
    unsafe {
        let mut _local0 = bumpalo::Bump::with_capacity(3400217169701580591);
        bumpalo::Bump::iter_allocated_chunks_raw(&(_local0));
        bumpalo::Bump::chunk_capacity(&(_local0));
    }
}
