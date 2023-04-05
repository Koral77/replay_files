extern crate unicode_segmentation;

fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(8463800222054970741 ,8463951407229173877 ,false);
    let _ = unicode_segmentation::GraphemeCursor::is_boundary(&mut (_local0), "Ë", 8463800222054970740);
}
