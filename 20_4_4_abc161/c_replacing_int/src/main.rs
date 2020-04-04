use std::cmp;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let mut n: i64 = iter.next().unwrap().parse().unwrap();
    let k: i64 = iter.next().unwrap().parse().unwrap();

    // 0がでたら終了
    // 暫定Nと同じ値がでたら終了

    // 計算時間短縮の為、n/kの余りを計算
    n = n % k;

    let abs: i64 = (n - k).abs();

    println!("{}", cmp::min::<i64>(n, abs));
}
