extern crate unicode_segmentation;

fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(18446744073709551615 ,8502796096475496447 ,false);
    let _ = unicode_segmentation::GraphemeCursor::is_boundary(&mut (_local0), "\u{6dd}", 18446744073709551615);
}
