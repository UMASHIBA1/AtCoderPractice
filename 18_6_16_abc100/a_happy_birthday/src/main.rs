use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let a: i64 = iter.next().unwrap().parse().unwrap();
    let b: i64 = iter.next().unwrap().parse().unwrap();

    if a <= 8 && b <= 8 {
        println!("Yay!");
    } else {
        println!(":(");
    }
}
