use std::io;

fn main() {
    let target_vec: Vec<i64> = vec![
        1, 1, 1, 2, 1, 2, 1, 5, 2, 2, 1, 5, 1, 2, 1, 14, 1, 5, 1, 5, 2, 2, 1, 15, 2, 2, 5, 4, 1, 4,
        1, 51,
    ];

    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();
    let index = buffer.trim().parse::<usize>().unwrap();
    println!("{}", target_vec[index - 1]);
}
