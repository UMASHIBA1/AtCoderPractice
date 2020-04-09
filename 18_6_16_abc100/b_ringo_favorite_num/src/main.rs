use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let d: i64 = iter.next().unwrap().parse().unwrap();
    let n: i64 = iter.next().unwrap().parse().unwrap();

    let base_num: i64 = 100i64.pow(d as u32);

    if n == 100 {
        println!("{}", base_num * n + base_num);
    } else {
        println!("{}", base_num * n);
    }
}
