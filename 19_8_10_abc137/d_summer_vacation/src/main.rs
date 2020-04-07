use std::collections::BinaryHeap;
use std::io;

struct PartTimeJob {
    a: i64,
    b: i64,
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut line_one_iter = buf.split_whitespace();

    let n: i64 = line_one_iter.next().unwrap().parse().unwrap();
    let m: i64 = line_one_iter.next().unwrap().parse().unwrap();

    let mut part_time_jobs_list: Vec<BinaryHeap<i64>> = Vec::new(); //part_time_jobs_list[0] = 残り日数1日のキュー
    for _ in 0..(m - 1) {
        let queue: BinaryHeap<i64> = BinaryHeap::new();
        part_time_jobs_list.push(queue);
    }

    for _ in 0..n {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        let mut iter = buf.split_whitespace();

        let a: i64 = iter.next().unwrap().parse().unwrap();
        let b: i64 = iter.next().unwrap().parse().unwrap();
        println!("a: {}", a);
        println!("b: {}", b);

        let part_time_job = PartTimeJob { a: a, b: b };

        for i in 1..m {
            if part_time_job.a <= i {
                part_time_jobs_list[(i - 1) as usize].push(part_time_job.b);
            }
        }
    }

    let mut total_salary: i64 = 0;

    for i in &mut part_time_jobs_list {
        if !i.is_empty() {
            total_salary += &i.pop().unwrap();
        }
    }

    println!("{}", total_salary);

    // part_time_jobs_list.sort_by(|a, b| a.b.cmp(&b.b));
}
