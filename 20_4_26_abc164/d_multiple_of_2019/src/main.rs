// 参考: https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

fn main() {
    input! {
        cs: chars
    };

    let cs_len = cs.len();

    if cs_len < 4 {
        println!("0");
        return;
    }

    let i64_cs: Vec<i64> = {
        let mut i64_cs: Vec<i64> = Vec::new();
        for i in &cs {
            i64_cs.push(*i as i64 - 48);
        }

        i64_cs
    };

    let mut target_count = 0;


    for digit_count in 4..(cs_len + 1) {
        for start_index in 0..(cs_len - digit_count + 1) { // start_index変える
            let mut target_num = 0;
            for i in 0..digit_count {
                target_num += i64_cs[start_index + i] * i64::pow(10, (digit_count - (i + 1)) as u32); // 各桁の数字足す
            }
    
            if target_num % 2019 == 0 {
                target_count += 1;
            }
        }
    }

    println!("{}", target_count);

}
