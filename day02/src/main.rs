use utils::read_lines;

fn main() {
    println!("day02");
    part1();
    part2();
}

fn part1() {
    let mut total_possible_games = 0;

    let games = read_lines!("input.txt");
    'games: for (idx, line) in games.into_iter().enumerate() {
        let (_, game) = line.split_once(": ").unwrap();
        let sets = game.split("; ");
        for set in sets {
            let blocks = set.split(", ");
            for block in blocks {
                let (count, color) = block.split_once(" ").unwrap();
                let max = match color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => panic!("unknown color"),
                };
                if count.parse::<i32>().unwrap() > max {
                    continue 'games;
                }
            }
        }

        total_possible_games += idx + 1;
    }

    println!(" - part1: {}", total_possible_games); // 2377
}

fn part2() {
    let mut sum_game_powers = 0;
    let lines = read_lines!("input.txt");
    for line in lines {
        let (_, game) = line.split_once(": ").unwrap();

        let mut game_max = [0, 0, 0];

        let sets = game.split("; ");
        for set in sets {
            let blocks: Vec<&str> = set.split(", ").collect();
            for block in &blocks {
                let (count_str, color) = block.split_once(" ").unwrap();
                let count = count_str.parse::<u32>().unwrap();

                match color {
                    "red" => game_max[0] = game_max[0].max(count),
                    "green" => game_max[1] = game_max[1].max(count),
                    "blue" => game_max[2] = game_max[2].max(count),
                    _ => panic!("unknown color"),
                };
            }
        }

        let game_power = game_max[0] * game_max[1] * game_max[2];
        sum_game_powers += game_power;
    }

    println!(" - part2: {}", sum_game_powers); // 71220
}
