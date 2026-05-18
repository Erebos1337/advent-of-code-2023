use utils::{boxed_grid, build_char_grid};

fn main() {
    println!("day03");
    part1();
    part2();
}

fn part1() {
    let char_grid = build_char_grid!("input.txt");
    let height = char_grid.len();
    let width = char_grid[0].len();

    let mut part_nums_grid = boxed_grid(height, width, false);
    for (y, row) in char_grid.iter().enumerate() {
        for (x, &char) in row.iter().enumerate() {
            if !char.is_ascii_digit() && char != '.' {
                mark_part_nums(&mut part_nums_grid, y, x, height, width);
            }
        }
    }

    let mut sum_part_nums = 0;
    for y in 0..height {
        let mut curr_num: u32 = 0;
        let mut is_part_num = false;
        for x in 0..width {
            let char = char_grid[y][x];
            if char.is_ascii_digit() {
                curr_num = curr_num * 10 + char.to_digit(10).unwrap();
                is_part_num |= part_nums_grid[y][x];
            } else {
                if is_part_num {
                    sum_part_nums += curr_num;
                }
                curr_num = 0;
                is_part_num = false;
            }
        }
        if is_part_num {
            sum_part_nums += curr_num;
        }
    }
    println!(" - part1: {}", sum_part_nums); // 539433
}

fn part2() {
    let char_grid = build_char_grid!("input.txt");
    let height = char_grid.len();
    let width = char_grid[0].len();

    let mut gears: Vec<(usize, usize)> = Vec::new();
    for (y, row) in char_grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == '*' {
                gears.push((y, x));
            }
        }
    }

    let mut sum_gear_ratios: u32 = 0;
    for gear_location in gears {
        sum_gear_ratios += calc_gear_ratio(gear_location, &char_grid, height, width);
    }
    println!(" - part2: {}", sum_gear_ratios); // 467835
}

fn mark_part_nums(
    part_nums_grid: &mut [Box<[bool]>],
    my: usize,
    mx: usize,
    height: usize,
    width: usize,
) {
    let min_y = my.saturating_sub(1);
    let max_y = usize::min(my + 1, height - 1);
    let min_x = mx.saturating_sub(1);
    let max_x = usize::min(mx + 1, width - 1);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            part_nums_grid[y as usize][x as usize] = true;
        }
    }
}

fn calc_gear_ratio(
    (my, mx): (usize, usize),
    char_grid: &[Box<[char]>],
    height: usize,
    width: usize,
) -> u32 {
    let min_y = my.saturating_sub(1);
    let max_y = usize::min(my + 1, height - 1);
    let min_x = mx.saturating_sub(1);
    let max_x = usize::min(mx + 1, width - 1);

    let mut part_locations: Vec<(usize, usize)> = Vec::new();
    for y in min_y..=max_y {
        let mut is_neighbor_num = false;
        for x in min_x..=max_x {
            let char = char_grid[y][x];
            if char.is_ascii_digit() {
                if !is_neighbor_num {
                    part_locations.push((y, x));
                }
                is_neighbor_num = true;
            } else {
                is_neighbor_num = false;
            }
        }
    }

    let mut gear_ratio: u32 = 0;
    if part_locations.len() == 2 {
        let num1 = collect_part_num(part_locations[0], char_grid, width);
        let num2 = collect_part_num(part_locations[1], char_grid, width);
        gear_ratio = num1 * num2;
    }
    gear_ratio
}

fn collect_part_num((my, mx): (usize, usize), char_grid: &[Box<[char]>], width: usize) -> u32 {
    let mut num: u32 = 0;
    let mut min_x = mx;
    while min_x > 0 && char_grid[my][min_x - 1].is_ascii_digit() {
        min_x = min_x - 1;
    }
    let mut x = min_x;
    while x < width && char_grid[my][x].is_ascii_digit() {
        num = num * 10 + char_grid[my][x].to_digit(10).unwrap();
        x += 1;
    }
    num
}
