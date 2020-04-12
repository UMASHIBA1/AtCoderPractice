use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let s: Vec<String> = {
        buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.trim().chars().map(|c| c.to_string()).collect()
    };

    let mut count = 0;

    for i in 0..n {
        for j in i + 1..n {
            if s[i] != s[j] {
                let sub_j_i = j - i;
                for k in j + 1..n {
                    if s[i] != s[k] && s[j] != s[k] {
                        let sub_k_j = k - j;
                        if sub_k_j != sub_j_i {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}
