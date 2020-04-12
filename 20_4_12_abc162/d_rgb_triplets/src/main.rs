use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let s: Vec<char> = {
        buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.trim().chars().collect()
    };

    let mut r_count = 0;
    let mut g_count = 0;
    let mut b_count = 0;

    for i in 0..n {
        match s[i] {
            'R' => r_count += 1,
            'G' => g_count += 1,
            'B' => b_count += 1,
            _ => (),
        }
    }

    let count = r_count * g_count * b_count;

    let mut minus_count = 0;
    for i in 0..n {
        let mut j = i + 1;
        let mut k = i + 2;
        while k < n {
            if s[i] != s[j] && s[j] != s[k] && s[i] != s[k] {
                minus_count += 1;
            }
            j += 1;
            k = j + (j - i);
        }
    }

    println!("{}", count - minus_count);
}
