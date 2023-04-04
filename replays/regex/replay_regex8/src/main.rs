extern crate regex;

fn main() {
    let _local0 = regex::bytes::RegexSet::empty();
    let _local1 = regex::bytes::RegexSet::matches(&(_local0), &[137, 137, 137, 137, 137, 137, 137, 137, 137, 137, 137, 137, 137, 137, 137, 138]);
    regex::bytes::SetMatches::matched(&(_local1), 9910603678816504201);
}
