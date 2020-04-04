use std::collections::VecDeque;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let k: i64 = buf.trim().parse().unwrap();
    let mut queue: VecDeque<i64> = VecDeque::new();

    for i in 1..10 {
        queue.push_back(i);
    }

    let mut count: i64 = 0;

    loop {
        // k番目のlunlunNumberかの判定処理
        let previous_digit_num: i64 = queue.pop_front().unwrap();
        count += 1;
        if count == k {
            println!("{}", previous_digit_num);
            break;
        }
        // lunlunNumberの追加処理
        let basic_digit: i64 = previous_digit_num * 10;
        let remainder: i64 = previous_digit_num % 10;
        if remainder != 0 {
            queue.push_back(basic_digit + remainder - 1);
        }

        queue.push_back(basic_digit + remainder);

        if remainder != 9 {
            queue.push_back(basic_digit + remainder + 1);
        }
    }
}
