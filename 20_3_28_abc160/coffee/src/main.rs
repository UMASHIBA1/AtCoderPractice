use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let input_vec: Vec<char> = buffer.chars().collect();

    if input_vec[2] == input_vec[3] && input_vec[4] == input_vec[5] {
        println!("Yes");
    } else {
        println!("No");
    }
}
