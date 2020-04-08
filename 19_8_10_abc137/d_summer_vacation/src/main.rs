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

    let mut part_time_jobs_list: Vec<PartTimeJob> = Vec::new();

    for _ in 0..n {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        let mut iter = buf.split_whitespace();

        let a: i64 = iter.next().unwrap().parse().unwrap();
        let b: i64 = iter.next().unwrap().parse().unwrap();

        let part_time_job = PartTimeJob { a: a, b: b };

        part_time_jobs_list.push(part_time_job);
    }
    part_time_jobs_list.sort_by(|a, b| a.a.cmp(&b.a));

    let mut total_salary = 0;
    let mut days_left = 1;
    let mut primary_queue: BinaryHeap<i64> = BinaryHeap::new();
    let mut loop_count: usize = 0;

    for _ in 0..m {
        while (loop_count as i64) < n && part_time_jobs_list[loop_count].a <= days_left {
            primary_queue.push(part_time_jobs_list[loop_count].b);
            loop_count += 1;
        }

        if let Some(salary) = primary_queue.pop() {
            total_salary += salary;
        }
        days_left += 1;
    }

    println!("{}", total_salary);
}
