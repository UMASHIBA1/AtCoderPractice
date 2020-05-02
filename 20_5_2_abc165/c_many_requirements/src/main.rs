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
        n: i64,
        m: i64,
        q: usize,
        abcd_list: [[i64; 4]; q]
    };

    let mut abcd_list = abcd_list;

    abcd_list.sort_by(|a, b| a[0].cmp(&b[0]));

    for i in &abcd_list {
        println!("{}", i[0]);
    }

    let mut a_list: Vec<i64> = vec![];

    let mut previous_equal_start_index: Vec<i64> = vec![];

    // for i in 0..q {
    //     abcd_list
    //     let a: Vec<i64> = Vec::new();
    //     let now_abcd = &abcd_list[i];
    //     let ai = now_abcd[0];
    //     let bi = now_abcd[1];
    //     let ci = now_abcd[2];
    //     let di = now_abcd[3];
    //     let mut now_a_ai = 1;
    //     while now_a_ai + ci < m {

    //     }
    // }
}
