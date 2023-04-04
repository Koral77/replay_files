extern crate byteorder;
use byteorder::ByteOrder;

fn main() {
    byteorder::BigEndian::read_u64(&[0, 0, 0, 64, 255]);
}
