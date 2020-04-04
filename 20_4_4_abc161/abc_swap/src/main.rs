use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let input_num_vec: Vec<i64> = buffer
        .trim()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    let first_num = input_num_vec[0];
    let second_num = input_num_vec[1];
    let third_num = input_num_vec[2];

    println!("{} {} {}", third_num, first_num, second_num);
}
