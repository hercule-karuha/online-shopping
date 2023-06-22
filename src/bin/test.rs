use md5;
fn main() {
    let digest = md5::compute("12345678");
    println!("{}", format!("{:x}", digest));
}
