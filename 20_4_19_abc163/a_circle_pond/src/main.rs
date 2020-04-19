use std::f64::consts::PI;
use std::io;
fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let r: f64 = buf.trim().parse().unwrap();

    println!("{}", 2.0 * r * PI);
}
