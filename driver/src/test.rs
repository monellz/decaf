use driver::*;

fn main() {
    for result in test_all("testcase/S1", Pa::Pa1b).unwrap() {
        println!("{:?}", result);
    }
}
