extern crate unicode_segmentation;

fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(4268070197446523713 ,4268070196471726080 ,false);
    unicode_segmentation::GraphemeCursor::prev_boundary(&mut (_local0), ";;;;;\n\n", 4268070197446523707);
    unicode_segmentation::GraphemeCursor::next_boundary(&mut (_local0), "\n\n\n\n\n\n\n\n", 4268070197446522939);
}
