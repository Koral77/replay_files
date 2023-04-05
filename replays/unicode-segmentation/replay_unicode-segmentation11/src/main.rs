extern crate unicode_segmentation;

fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(18446744073709551615 ,16212958658533785599 ,false);
    let _ = unicode_segmentation::GraphemeCursor::next_boundary(&mut (_local0), "0", 18446744073709551615);
}
