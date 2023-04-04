extern crate regex;

fn main() {
    let _local0 = regex::Regex::new("\0\0\0\0$");
    let _local1_param0_helper1 = _local0.unwrap();
    let _local1 = regex::Regex::find_at(&(_local1_param0_helper1), "\r\0\u{1}\u{e}a", 2449958197290798336);
    let _local2_param0_helper1 = _local1.unwrap();
    regex::Match::end(&(_local2_param0_helper1));
}
