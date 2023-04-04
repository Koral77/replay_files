extern crate unicode_segmentation;

fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(5404402016221612875 ,5425481077020773195 ,false);
    unicode_segmentation::GraphemeCursor::prev_boundary(&mut (_local0), "KKK", 5425512962414627659);
    unicode_segmentation::GraphemeCursor::next_boundary(&mut (_local0), "KKK1", 5425512838301707595);
}
