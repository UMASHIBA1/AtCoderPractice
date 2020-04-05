use std::collections::BinaryHeap;
use std::io;

fn main() {
    let n: i64 = {
        let mut line_one_buf = String::new();
        io::stdin().read_line(&mut line_one_buf).unwrap();
        let n: i64 = line_one_buf.trim().parse().unwrap();
        n
    };

    let mut red_balls: BinaryHeap<i64> = BinaryHeap::new();
    let mut blue_balls: BinaryHeap<i64> = BinaryHeap::new();

    for _ in 0..n {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let x: i64 = iter.next().unwrap().parse().unwrap();
        let c: String = iter.next().unwrap().parse().unwrap();
        if c == "R" {
            red_balls.push(x);
        } else if c == "B" {
            blue_balls.push(x);
        }
    }

    let mut sorted_red_balls = Vec::new();
    let mut sorted_blue_balls = Vec::new();

    // 処理
    loop {
        if red_balls.is_empty() {
            break;
        };
        let x = red_balls.pop().unwrap();
        sorted_red_balls.push(x);
    }

    loop {
        if blue_balls.is_empty() {
            break;
        };
        let x = blue_balls.pop().unwrap();
        sorted_blue_balls.push(x);
    }

    let red_len = sorted_red_balls.len();
    let blue_len = sorted_blue_balls.len();

    for i in 0..red_len {
        println!("{}", sorted_red_balls[red_len - i - 1]);
    }
    for i in 0..blue_len {
        println!("{}", sorted_blue_balls[blue_len - i - 1]);
    }
}
