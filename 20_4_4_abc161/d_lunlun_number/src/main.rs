use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let k: i64 = buf.trim().parse().unwrap();

    let mut count = 0;

    let mut now_num: i64 = 1;

    loop {
        // ルンルン数かの判定
        let mut is_lunlun_num = true;
        let str_now_num: String = now_num.to_string();
        let str_num_vec: Vec<i64> = str_now_num.chars().map(|c| c as i64).collect();
        let number_of_digits: usize = str_num_vec.len();

        if number_of_digits != 1 {
            // 二桁以上のルンルン数かの判定
            for i in 0..(number_of_digits - 1) {
                if (str_num_vec[i] - str_num_vec[i + 1]).abs() > 1 {
                    is_lunlun_num = false;
                    break;
                }
            }
        }

        if is_lunlun_num {
            count += 1;
            // println!("count: {}", count);
            // println!("now_num: {}", now_num);
        }

        if count == k {
            break;
        }

        now_num += 1;
    }

    println!("{}", now_num);
}
