use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let input_num: i32 = buffer.trim().parse().unwrap();

    let happy_1000 = input_num / 500 * 1000;
    let happy_5 = (input_num % 500) / 5 * 5;
    println!("{}", happy_1000 + happy_5);
}
