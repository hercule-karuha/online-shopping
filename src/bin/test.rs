use md5;
use std::time::{SystemTime};
fn main() {
    let digest = md5::compute("1234567890");
    let md5ps = format!("{:x}", digest).to_uppercase();
    println!("{},{:?}", md5ps,SystemTime::now());
}
