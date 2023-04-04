extern crate regex;

fn main() {
    let _local0 = regex::Regex::new("E\0|\u{741}");
    let _local1_param0_helper1 = _local0.unwrap();
    let _local1 = regex::Regex::capture_locations(&(_local1_param0_helper1));
    regex::CaptureLocations::get(&(_local1) ,9238929028971069751);
}
