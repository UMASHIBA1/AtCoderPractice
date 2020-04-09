use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let mut num_vec: Vec<i64> = Vec::new();

    for _ in 0..n {
        let num: i64 = iter.next().unwrap().parse().unwrap();

        num_vec.push(num);
    }

    let mut div2_count = 0;
    for num in &num_vec {
        let mut num = *num;
        while num % 2 == 0 {
            div2_count += 1;
            num /= 2;
        }
    }

    println!("{}", div2_count);
}
