extern crate byteorder;
use byteorder::ByteOrder;

fn main() {
    byteorder::LittleEndian::read_u64(&[11]);
}
