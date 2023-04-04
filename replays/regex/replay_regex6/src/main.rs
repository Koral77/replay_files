extern crate regex;

fn main() {
    let _local0 = regex::Regex::new("[\\\\-^]\0\0\0\0\0\0\0\0\0\0\u{1}\0\0\0\0\u{3}\0\0\0\0\u{1e}\0\u{3}r\u{1}\0\u{f}\0\u{3}r|\u{17}\u{10}\0\0r|\0\0\0%\u{17}\u{10}\0\0r|\0\0");
    let _local1_param0_helper1 = _local0.unwrap();
    let _local1 = regex::Regex::find_at(&(_local1_param0_helper1), "\0%r|\0r|\0\u{2}\0\u{3}\0\0|\u{17}\u{10}\0\0r|\0\0\u{1}\0\0\0\0\0\u{3}r|\u{17}\u{10}\0\0r\0\0\0\0\0#Y@\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}\u{1}", 15924992);
    let _local2_param0_helper1 = _local1.unwrap();
    regex::Match::end(&(_local2_param0_helper1));
}