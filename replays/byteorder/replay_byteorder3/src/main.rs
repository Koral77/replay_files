extern crate byteorder;
use byteorder::ByteOrder;

fn main() {
    byteorder::BigEndian::read_u16(&[33]);
}
