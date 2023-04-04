extern crate unicode_segmentation;

fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(5208492444341520467 ,3407250190757808200 ,true);
    unicode_segmentation::GraphemeCursor::prev_boundary(&mut (_local0), "HHHZ\\HHH\0\u{e040}HHK", 5208492444341520456);
    unicode_segmentation::GraphemeCursor::next_boundary(&mut (_local0), "\0\"H[HH(HHH?HH\u{e040}", 5192729847860321864);
}
