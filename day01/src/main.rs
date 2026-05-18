use utils::read_lines;

fn main() {
    println!("day01");
    part1();
    part2();
}

fn part1() {
    let lines = read_lines!("input.txt");
    let mut total = 0;
    for line in lines {
        let chars = line.chars().collect::<Vec<char>>();
        let mut first_digit = 0;
        let mut last_digit = 0;
        let mut found_first = false;
        for char in chars {
            if char.is_digit(10) {
                if !found_first {
                    first_digit = char.to_digit(10).unwrap();
                    found_first = true;
                }
                last_digit = char.to_digit(10).unwrap()
            }
        }
        total += 10 * first_digit + last_digit;
    }
    println!(" - part1: {}", total); // 55816
}

fn part2() {
    let digit_strings = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
    ];
    let mut total = 0;

    let lines = read_lines!("input.txt");
    for line in lines {
        let mut first_digit = 0;
        let mut last_digit = 0;
        let mut found_first = false;

        let mut rest_string = line.as_str();
        while !rest_string.is_empty() {
            let mut found_digit = false;
            let mut digit = 0;

            for digit_string in digit_strings {
                if rest_string.starts_with(digit_string) {
                    digit = num_string_to_digit(digit_string);
                    found_digit = true;
                    break;
                }
            }

            if found_digit {
                if !found_first {
                    first_digit = digit;
                    found_first = true;
                }
                last_digit = digit;
            }

            rest_string = &rest_string[1..];
        }
        total += 10 * first_digit + last_digit;
    }
    println!(" - part2: {}", total); // 54980
}

fn num_string_to_digit(num_string: &str) -> u32 {
    match num_string {
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}
