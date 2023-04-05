extern crate unicode_segmentation;

fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(4268070197446523707 ,4268070196469563392 ,false);
    let _ = unicode_segmentation::GraphemeCursor::next_boundary(&mut (_local0), "; ", 4268070197446523705);
}
