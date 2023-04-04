extern crate regex;

fn main() {
    let _local0 = regex::Regex::new("(\0\0\0\0\0\0\u{10}|\0\0\0\0\0)\0\0\0\0\0\0\0\0\0\u{10}|\0\0\0\0\0\0\0");
    let _local1_param0_helper1 = _local0.unwrap();
    let _local1 = regex::Regex::find_at(&(_local1_param0_helper1), "\0\u{4}\0*****\u{17}***************\0\0\0\0\0\0\0\0\0\0", 35184372153856);
    let _local2_param0_helper1 = _local1.unwrap();
    regex::Match::end(&(_local2_param0_helper1));
}
