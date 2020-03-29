use std::io;

fn main() {
    // 一行目を取得、分割
    let mut one_line_buffer = String::new();

    io::stdin().read_line(&mut one_line_buffer).unwrap();
    let one_line_vec: Vec<i64> = one_line_buffer
        .trim()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    let lake_circumference: i64 = one_line_vec[0];
    let house_num: usize = one_line_vec[1] as usize;

    // 二行目を取得、分割
    let mut two_line_buffer = String::new();
    io::stdin().read_line(&mut two_line_buffer).unwrap();

    let house_distances_from_north: Vec<i64> = two_line_buffer
        .trim()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    // for i in &house_distances_from_north {
    //     println!("{}", i);
    // }

    // 隣の家間の距離を計算
    let mut house_distances_from_previous: Vec<i64> = vec![];

    house_distances_from_previous.push(
        house_distances_from_north[0]
            + (lake_circumference - house_distances_from_north[house_num - 1]),
    );

    // 家間の最大距離とそのindex
    let mut max_distance: i64 = house_distances_from_previous[0];

    // println!("0: {}", house_distances_from_previous[0]);

    for i in 1..house_num {
        house_distances_from_previous
            .push(house_distances_from_north[i] - house_distances_from_north[i - 1]);

        if house_distances_from_previous[i] > max_distance {
            max_distance = house_distances_from_previous[i];
        }

        // println!("{}: {}", i, house_distances_from_previous[i]);
    }

    let mut distance_sum: i64 = 0;

    for i in house_distances_from_previous {
        distance_sum += i;
    }

    println!("{}", distance_sum - max_distance);
}
