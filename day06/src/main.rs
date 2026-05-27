use utils::read_lines;

fn main() {
    println!("day06");
    part1();
    part2();
}

fn part1() {
    let mut lines = read_lines!("input.txt");

    let times = parse_num_row(&lines.next().unwrap());
    let records = parse_num_row(&lines.next().unwrap());

    let product_winning_options: u64 = times
        .iter()
        .zip(records.iter())
        .map(|(time, record)| find_winning_options(*time, *record))
        .product();

    println!(" - part1: {}", product_winning_options); // 3317888
}

fn part2() {
    let mut lines = read_lines!("input.txt");

    let times = parse_num_row(&lines.next().unwrap());
    let records = parse_num_row(&lines.next().unwrap());

    let time = merge_numbers(times);
    let record = merge_numbers(records);

    let num_winning_options = find_winning_options(time, record);

    println!(" - part2: {}", num_winning_options); // 24655068
}

fn parse_num_row(line: &str) -> Box<[u64]> {
    let (_, nums_str) = line.split_once(":").unwrap();
    let nums = nums_str
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    nums
}

fn find_winning_options(time: u64, record: u64) -> u64 {
    let mut num_winning_options = time - 1;

    for time_charging in 1..time {
        let distance = calc_distance(time_charging, time);
        if distance > record {
            num_winning_options -= time_charging - 1;
            break;
        }
    }

    for time_charging in 1..time {
        let distance = calc_distance(time - time_charging, time);
        if distance > record {
            num_winning_options -= time_charging - 1;
            break;
        }
    }

    num_winning_options
}

fn calc_distance(time_charging: u64, time_total: u64) -> u64 {
    (time_total - time_charging) * time_charging
}

fn merge_numbers(numbers: Box<[u64]>) -> u64 {
    numbers
        .into_iter()
        .fold(String::new(), |acc, num| acc + num.to_string().as_str())
        .parse()
        .unwrap()
}
