use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: i64 = buf.trim().parse().unwrap();

    let mut count: i64 = 0;

    for i in 1..n + 1 {
        if i % 3 != 0 && i % 5 != 0 {
            count += i;
        }
    }

    println!("{}", count);
}
