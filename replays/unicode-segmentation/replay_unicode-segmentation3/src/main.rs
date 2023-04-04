extern crate unicode_segmentation;

fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(18446707789825836799 ,18446744073709551615 ,false);
    unicode_segmentation::GraphemeCursor::provide_context(&mut (_local0), "1", 18446744073709551615);
}
