use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let input_vec: Vec<f64> = buffer
        .trim()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    let a_input = input_vec[0];
    let b_input = input_vec[1];
    let c_input = input_vec[2];
    println!("a input: {}", a_input);
    println!("b input: {}", b_input);
    println!("c input: {}", c_input);

    if (a_input.sqrt() + b_input.sqrt()) < c_input.sqrt() {
        println!("Yes");
    } else {
        println!("No");
    }

    println!("{}", 249999999_f64.sqrt());
    println!("{}", 250000000_f64.sqrt());
    println!("{}", 999999998_f64.sqrt());
}
