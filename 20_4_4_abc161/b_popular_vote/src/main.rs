use std::io;

fn main() {
    // 一行目を取得、分割
    let mut one_line_buffer = String::new();

    io::stdin().read_line(&mut one_line_buffer).unwrap();
    let one_line_vec: Vec<i64> = one_line_buffer
        .trim()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    let popular_product_num = one_line_vec[1]; //M

    // 二行目を取得、分割
    let mut two_line_buffer = String::new();
    io::stdin().read_line(&mut two_line_buffer).unwrap();

    let mut vote_nums: Vec<i64> = two_line_buffer
        .trim()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    // 処理

    let total_vote_num = {
        let mut total: i64 = 0;
        for i in &vote_nums {
            total = &total + i;
        }
        total
    };

    println!("total: {}", &total_vote_num);

    let judge_line_num = &total_vote_num / (4 * &popular_product_num);

    println!("jduge: {}", &judge_line_num);
    vote_nums.sort_by(|a, b| b.cmp(a));

    for i in &vote_nums {
        println!("{}", i);
    }

    let popular_product_votes: Vec<i64> = vote_nums[..(popular_product_num as usize)].to_vec();

    let mut is_yes = true;
    for i in &popular_product_votes {
        if i < &judge_line_num {
            is_yes = false;
        }
    }
    if is_yes {
        println!("Yes");
    } else {
        println!("No");
    }
}
