use driver::*;

fn main() {
    for result in test_all("testcase/S4", Pa::Pa5).unwrap() {
        println!("{:?}", result);
    }
}
