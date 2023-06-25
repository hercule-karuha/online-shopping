use md5;
fn main() {
    let digest = md5::compute("1234567890");
    let md5ps = format!("{:x}", digest).to_uppercase();
    println!("{}", md5ps);
}
