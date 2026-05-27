use utils::read_lines;

fn main() {
    println!("day05");
    part1();
    part2();
}

fn part1() {
    let blocks = read_blocks("input.txt");

    let seeds = parse_seeds(&blocks[0]);

    let layers = [
        parse_number_block_with_header(&blocks[1]),
        parse_number_block_with_header(&blocks[2]),
        parse_number_block_with_header(&blocks[3]),
        parse_number_block_with_header(&blocks[4]),
        parse_number_block_with_header(&blocks[5]),
        parse_number_block_with_header(&blocks[6]),
        parse_number_block_with_header(&blocks[7]),
    ];

    let min_location = seeds
        .iter()
        .map(|seed| {
            layers
                .iter()
                .fold(*seed, |prev, layer| apply_mappings(prev, &layer))
        })
        .min()
        .unwrap();

    println!(" - part1: {}", min_location); // 88151870
}

fn part2() {
    let blocks = read_blocks("input.txt");

    let seed_ranges = parse_seed_ranges(&blocks[0]);

    let layers = [
        parse_range_rules(&blocks[1]),
        parse_range_rules(&blocks[2]),
        parse_range_rules(&blocks[3]),
        parse_range_rules(&blocks[4]),
        parse_range_rules(&blocks[5]),
        parse_range_rules(&blocks[6]),
        parse_range_rules(&blocks[7]),
    ];

    let min_location = seed_ranges
        .iter()
        .map(|seed_range| find_min_location(*seed_range, 0, &layers))
        .min()
        .unwrap();

    println!(" - part2: {}", min_location); // 2008785
}

fn find_min_location(
    source_range: (u64, u64),
    level: usize,
    layers: &[Box<[((u64, u64), i64)]>],
) -> u64 {
    if level == layers.len() {
        return source_range.0;
    }

    let min_location = layers[level]
        .iter()
        .filter(|(rule_range, _)| do_ranges_overlap(source_range, *rule_range))
        .map(|(rule_range, delta)| {
            let overlapping_range = (
                source_range.0.max(rule_range.0),
                source_range.1.min(rule_range.1),
            );
            let target_range = (
                overlapping_range.0.strict_add_signed(*delta),
                overlapping_range.1.strict_add_signed(*delta),
            );
            find_min_location(target_range, level + 1, layers)
        })
        .min()
        .unwrap_or(u64::MAX);

    min_location
}

fn read_blocks(filename: &str) -> Box<[String]> {
    let lines = read_lines!(filename);
    let mut blocks = Vec::new();

    let mut current_block = vec![];
    for line in lines {
        if !line.trim().is_empty() {
            current_block.push(line);
        } else {
            blocks.push(current_block.join("\n"));
            current_block.clear();
        }
    }
    blocks.push(current_block.join("\n"));

    blocks.into_boxed_slice()
}

fn parse_seeds(block: &str) -> Box<[u64]> {
    let (_, seeds_str) = block.split_once(": ").unwrap();
    let seeds = seeds_str.split(" ").map(|s| s.parse().unwrap()).collect();
    return seeds;
}

fn parse_seed_ranges(block: &str) -> Box<[(u64, u64)]> {
    parse_seeds(block)
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1] - 1))
        .collect()
}

fn parse_number_block_with_header(block: &str) -> Box<[[u64; 3]]> {
    let mut rows: Vec<[u64; 3]> = Vec::new();
    for line in block.lines().skip(1) {
        let numbers = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        rows.push(numbers);
    }
    rows.into_boxed_slice()
}

fn parse_range_rules(block: &str) -> Box<[((u64, u64), i64)]> {
    let raw_mappings = parse_number_block_with_header(block);
    raw_mappings
        .iter()
        .map(|raw_mapping| {
            (
                (raw_mapping[1], raw_mapping[1] + raw_mapping[2] - 1),
                (raw_mapping[0].cast_signed()) - (raw_mapping[1].cast_signed()),
            )
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn map_value(value: u64, mapping: &[u64; 3]) -> Option<u64> {
    let target_min = mapping[0];
    let source_min = mapping[1];
    let source_len = mapping[2];
    if value >= source_min {
        let offset = value - source_min;
        if offset < source_len {
            return Some(target_min + offset);
        }
    }
    None
}

fn apply_mappings(value: u64, mappings: &[[u64; 3]]) -> u64 {
    for mapping in mappings.iter() {
        if let Some(mapped_value) = map_value(value, mapping) {
            return mapped_value;
        }
    }
    value
}

fn do_ranges_overlap(range1: (u64, u64), range2: (u64, u64)) -> bool {
    range1.0 <= range2.1 && range1.1 >= range2.0
}
