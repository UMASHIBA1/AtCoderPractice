use std::io;

fn is_lunlun_num(target_num: i64) -> bool {
    let mut is_lunlun_num: bool = true;
    let str_num: String = target_num.to_string();
    let str_num_vec: Vec<char> = str_num.chars().collect();
    let digit_num = str_num_vec.len();
    for i in 0..(digit_num - 1) {
        let now_num = str_num_vec[i] as i64;
        let next_num = str_num_vec[i + 1] as i64;
        if (now_num - next_num).abs() > 1 {
            is_lunlun_num = false;
            break;
        }
    }
    is_lunlun_num
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let k: i64 = buf.trim().parse().unwrap();

    let mut previous_digit_lunlun_nums: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut lunlun_num_count: i64 = 9; //これがkになったらprint

    if k <= 9 {
        println!("{}", &previous_digit_lunlun_nums[k as usize - 1]);
    } else {
        let mut now_digit_lunlun_nums: Vec<i64> = vec![]; //10,11,12,21,22,23

        loop {
            for i in &previous_digit_lunlun_nums {
                let basic_digit: i64 = i * 10; //10,20
                for f in 0..10 {
                    let now_num: i64 = basic_digit + f; //10,11,12,13
                    if is_lunlun_num(now_num) {
                        lunlun_num_count += 1;

                        if lunlun_num_count == k {
                            println!("{}", now_num);
                            return;
                        }
                        &now_digit_lunlun_nums.push(now_num);
                    }
                }
            }
            // 桁の更新
            previous_digit_lunlun_nums = now_digit_lunlun_nums.clone();
            now_digit_lunlun_nums = vec![];
        }
    }
}
