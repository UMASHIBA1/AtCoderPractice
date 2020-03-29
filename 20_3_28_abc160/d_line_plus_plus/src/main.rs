use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let target_input: Vec<i64> = buffer
        .trim()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    for i in target_input {
        println!("{}", i);
    }
}
