extern crate unicode_segmentation;

fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(5208492444341520456 ,5208492444341520431 ,true);
    unicode_segmentation::GraphemeCursor::prev_boundary(&mut (_local0), "HHHHHHHHHjHHH", 5208492444341520456);
    unicode_segmentation::GraphemeCursor::next_boundary(&mut (_local0), "HHHHHHHHHHHHH", 5208492589950978632);
}
