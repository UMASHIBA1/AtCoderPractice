use std::io;

fn main() {
    let mut target_str = String::new();
    io::stdin().read_line(&mut target_str).unwrap();
    let target_str = target_str.trim();

    let mut count: usize = 0;

    loop {
        match target_str.chars().nth(count) {
            Some('h') => match target_str.chars().nth(count + 1) {
                Some('i') => {
                    count += 2;
                }
                _ => {
                    println!("No");
                    break;
                }
            },
            None => {
                println!("Yes");
                break;
            }
            _ => {
                println!("No");
                break;
            }
        };
    }
}
