use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let mut s: i64 = iter.next().unwrap().parse().unwrap();
    let l: i64 = iter.next().unwrap().parse().unwrap();
    let r: i64 = iter.next().unwrap().parse().unwrap();

    loop {
        if s >= l && s <= r {
            println!("{}", s);
            break;
        }
        if s < l {
            s += 1;
        } else if s > r {
            s -= 1;
        }
    }
}
