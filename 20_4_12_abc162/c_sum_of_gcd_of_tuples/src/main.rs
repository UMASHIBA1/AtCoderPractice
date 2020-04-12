use std::cmp;
use std::io;

fn calc_gcd(num1: i64, num2: i64) -> i64 {
    let mut big_num = cmp::max(num1, num2);
    let mut min_num = cmp::min(num1, num2);

    let mut rest_num;
    loop {
        rest_num = big_num % min_num;
        if rest_num == 0 {
            return min_num;
        } else {
            big_num = min_num;
            min_num = rest_num;
        }
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let k: i64 = buf.trim().parse().unwrap();

    let mut total: i64 = 0;

    for num1 in 1..k + 1 {
        for num2 in 1..k + 1 {
            let gcd_num1_num2 = calc_gcd(num1, num2);
            for num3 in 1..k + 1 {
                total += calc_gcd(gcd_num1_num2, num3);
            }
        }
    }
    println!("{}", total);
}
