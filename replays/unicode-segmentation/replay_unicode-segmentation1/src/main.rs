extern crate unicode_segmentation;

fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(18446742978509668351 ,18446744073709551615 ,false);
    unicode_segmentation::GraphemeCursor::is_boundary(&mut (_local0), "t\u{7f}", 18446744073709551615);
}
