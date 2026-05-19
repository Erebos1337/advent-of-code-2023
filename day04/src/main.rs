use utils::read_lines;

fn main() {
    println!("day04");
    part1();
    part2();
}

fn part1() {
    let mut sum_card_values = 0;

    let lines = read_lines!("input.txt");
    for line in lines {
        let (_, numbers_str) = line.split_once(": ").unwrap();
        let (winning_numbers_str, have_numbers_str) = numbers_str.split_once(" | ").unwrap();
        let winning_numbers: Box<[u32]> = winning_numbers_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let have_numbers: Box<[u32]> = have_numbers_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut card_value = 0;
        for &num in have_numbers.iter() {
            if winning_numbers.contains(&num) {
                if card_value == 0 {
                    card_value = 1;
                } else {
                    card_value *= 2;
                }
            }
        }
        sum_card_values += card_value;
    }
    println!(" - part1: {}", sum_card_values); // 24733
}

fn part2() {
    let lines: Vec<String> = read_lines!("input.txt").collect();
    let mut cards: Box<[usize]> = vec![1; lines.len()].into_boxed_slice();

    let mut num_cards = 0;
    for (idx, line) in lines.iter().enumerate() {
        num_cards += cards[idx];

        let (_, numbers_str) = line.split_once(": ").unwrap();
        let (winning_numbers_str, have_numbers_str) = numbers_str.split_once(" | ").unwrap();
        let winning_numbers: Box<[u32]> = winning_numbers_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let have_numbers: Box<[u32]> = have_numbers_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut num_matches = 0;
        for &num in have_numbers.iter() {
            if winning_numbers.contains(&num) {
                num_matches += 1;
            }
        }

        for i in idx + 1..=idx + num_matches {
            cards[i] += cards[idx];
        }
    }
    println!(" - part2: {}", num_cards); // 5422730
}
