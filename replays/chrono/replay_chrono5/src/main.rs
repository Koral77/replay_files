extern crate chrono;

fn main() {
    let _local0 = chrono::naive::NaiveDateTime::from_timestamp_opt(-502509993984, 64);
    let _local1_param0_helper1 = _local0.unwrap();
    chrono::Datelike::with_ordinal0(&(_local1_param0_helper1), 4294967295);
}
