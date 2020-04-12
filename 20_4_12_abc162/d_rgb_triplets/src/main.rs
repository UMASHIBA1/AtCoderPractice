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

    let mut r_list: Vec<usize> = Vec::new();
    let mut g_list: Vec<usize> = Vec::new();
    let mut b_list: Vec<usize> = Vec::new();

    for i in 0..n {
        if s[i] == "R" {
            r_list.push(i);
        } else if s[i] == "G" {
            g_list.push(i);
        } else if s[i] == "B" {
            b_list.push(i);
        }
    }

    let r_len = r_list.len();
    let g_len = g_list.len();
    let b_len = g_list.len();

    let mut g_start_index = 0;
    let mut b_start_index = 0;

    let mut count = 0;

    for i in 0..r_len {
        for j in g_start_index..g_len {
            if g_list[j] > r_list[i] {
                let sub_j_i = g_list[j] - r_list[i];
                for k in b_start_index..b_len {
                    println!(
                        "i: {}, j: {}, k: {}, r_list: {}, g_list: {}, b_list: {}",
                        i, j, k, r_list[i], g_list[j], b_list[k]
                    );
                    if b_list[k] > g_list[j] {
                        let sub_k_j = b_list[k] - g_list[j];
                        if sub_k_j != sub_j_i {
                            count += 1;
                        }
                    } else {
                        b_start_index = k + 1;
                    }
                }
            } else {
                g_start_index = j + 1;
            }
        }
    }

    println!("{}", count);
}
