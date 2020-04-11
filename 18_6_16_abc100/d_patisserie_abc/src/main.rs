use std::collections::BinaryHeap;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();

    let n: i64 = iter.next().unwrap().parse().unwrap();
    let m: i64 = iter.next().unwrap().parse().unwrap();

    let mut primary_queue: BinaryHeap<i64> = BinaryHeap::new();

    for _ in 0..n {
        buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let x: i64 = iter.next().unwrap().parse().unwrap(); // 綺麗さ
        let y: i64 = iter.next().unwrap().parse().unwrap(); // 美味しさ
        let z: i64 = iter.next().unwrap().parse().unwrap(); // 人気度
        let sum_value: i64 = x + y + z;
        primary_queue.push(sum_value);
    }

    let mut total_value = 0;

    for _ in 0..m {
        total_value += primary_queue.pop().unwrap();
    }

    println!("{}", total_value);
}
