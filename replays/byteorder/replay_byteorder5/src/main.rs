extern crate byteorder;
use byteorder::ByteOrder;

fn main() {
    byteorder::LittleEndian::read_u32(&[50, 30, 50]);
}
