use std::fmt::Debug;
use std::io;
use std::str::FromStr;

fn read_and_divide_line<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut target_line = String::new();
    io::stdin().read_line(&mut target_line).unwrap();
    target_line
        .trim()
        .split_whitespace()
        .map(|c| T::from_str(c).unwrap())
        .collect()
}

fn main() {
    let kind_num: Vec<i64> = read_and_divide_line();
    let coupon_kind_num = kind_num[2];

    let refrigerator_prices: Vec<i64> = read_and_divide_line();
    let microwave_prices: Vec<i64> = read_and_divide_line();

    let mut each_price_combination: Vec<Vec<i64>> = vec![];

    for _ in 0..coupon_kind_num {
        each_price_combination.push(read_and_divide_line());
    }

    let min_refrigerator_price = {
        let mut tmp_refrigerator_prices = refrigerator_prices.clone();
        tmp_refrigerator_prices.sort();
        tmp_refrigerator_prices[0]
    };

    let min_microwave_price = {
        let mut tmp_microwave_prices = microwave_prices.clone();
        tmp_microwave_prices.sort();
        tmp_microwave_prices[0]
    };

    let mut min_price = min_refrigerator_price + min_microwave_price;

    for combination in each_price_combination {
        let now_price = refrigerator_prices[(combination[0] as usize) - 1]
            + microwave_prices[(combination[1] as usize) - 1]
            - combination[2];
        if now_price < min_price {
            min_price = now_price;
        }
    }

    println!("{}", min_price);
}
