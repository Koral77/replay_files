extern crate bytes;

fn main() {
    unsafe {
        let mut _local0 = bytes::BytesMut::zeroed(134217983);
        bytes::buf::BufMut::advance_mut(&mut (_local0), 18446744073707388928);
    }
}
