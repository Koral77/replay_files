extern crate regex;

fn main() {
    let _local0 = regex::Regex::new("$$$$$$$$$$");
    let _local1_param0_helper1 = _local0.unwrap();
    let _local1 = regex::Regex::find_at(&(_local1_param0_helper1), "$$$$$$$$$$$", 2604246222170760228);
    let _local2_param0_helper1 = _local1.unwrap();
    regex::Match::end(&(_local2_param0_helper1));
}
