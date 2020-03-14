use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let input_vec: Vec<i64> = buffer
        .trim()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    let height = input_vec[0];
    let width = input_vec[1];
    if height == 1 || width == 1 {
        println!("{}", 1);
    } else {
        println!("{}", (width * height + 1) / 2);
    }
}
