extern crate regex;

fn main() {
    let _local0 = regex::RegexBuilder::new("$");
    let _local1 = regex::RegexBuilder::build(&(&_local0));
    let _local2_param0_helper1 = _local1.unwrap();
    regex::Regex::shortest_match_at(&(_local2_param0_helper1), "{S", 8897841259371199355);
}
