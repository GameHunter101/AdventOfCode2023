use std::collections::HashSet;

fn main() {
    let input = String::from_utf8(std::fs::read("./src/input.txt").unwrap()).unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    let mut sum = 0;
    let mut all_match_counts = [0; 220];
    for (i, line) in lines.iter().enumerate() {
        let (winning_numbers, my_numbers) = parse_line(line);
        let match_count = get_matches(winning_numbers, my_numbers);
        all_match_counts[i] = match_count;
    }
    let mut all_copies = [0; 220];
    for (i, count) in all_match_counts.iter().enumerate() {
        if *count > 0 {
            for copy in i + 1..=i + *count {
                all_copies[copy] += 1;
            }
        }
    }
    for i in 0..220 {
        sum += count_copies(all_match_counts[i], i, &all_match_counts);
    }
    println!("{sum}");
}

fn parse_line(line: &str) -> ([&str; 10], [&str; 25]) {
    let (left, my_numbers_string) = line.split_once(" | ").unwrap();
    let winning_numbers_string = left.split_once(": ").unwrap().1;
    let winning_numbers: [&str; 10] = winning_numbers_string
        .split(" ")
        .filter(|num| num.len() > 0)
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();
    let my_numbers: [&str; 25] = my_numbers_string
        .split(" ")
        .filter(|num| num.len() > 0)
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();
    (winning_numbers, my_numbers)
}

fn get_matches(winning_numbers: [&str; 10], my_numbers: [&str; 25]) -> usize {
    let winning_set = HashSet::from(winning_numbers);
    let my_set = HashSet::from(my_numbers);

    winning_set.intersection(&my_set).collect::<Vec<_>>().len() as usize
}

fn count_copies(num: usize, index: usize, match_counts: &[usize]) -> u32 {
    if num == 0 {
        return 1;
    }
    let mut sum = 1;
    for i in index + 1..=index + num {
        let count = count_copies(match_counts[i], i, match_counts);
        sum += count;
    }
    return sum;
}
