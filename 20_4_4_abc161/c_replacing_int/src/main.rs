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

    let mut min_n: i64 = n;

    loop {
        let abs: i64 = (n - k).abs();
        if abs == 0 {
            min_n = abs;
            break;
        } else if abs == min_n {
            break;
        } else if abs < n {
            min_n = abs;
        }
        n = abs;
    }

    println!("{}", min_n);
}
