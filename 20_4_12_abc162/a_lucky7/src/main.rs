use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf = buf.trim().to_string();
    let mut num_vec: Vec<i64> = Vec::new();
    let mut is_seven_flag = false;

    for num in buf.chars() {
        let num_i64: i64 = num as i64 - 48;
        num_vec.push(num_i64);
    }

    for i in num_vec {
        if i == 7 {
            is_seven_flag = true;
            break;
        }
    }
    if is_seven_flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
