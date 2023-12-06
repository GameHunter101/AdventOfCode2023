use std::collections::HashSet;

fn main() {
    let input = String::from_utf8(std::fs::read("./src/input.txt").unwrap()).unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    let mut sum = 0;
    for line in lines {
        let (winning_numbers, my_numbers) = parse_line(line);
        let match_count = get_matches(winning_numbers, my_numbers);
        let exp = match_count as i32 - 1;
        if exp >= 0 {
            let score = 2_i32.pow(exp as u32);
            sum += score;
        }
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

fn get_matches(winning_numbers: [&str; 10], my_numbers: [&str; 25]) -> u32 {
    let winning_set = HashSet::from(winning_numbers);
    let my_set = HashSet::from(my_numbers);

    winning_set.intersection(&my_set).collect::<Vec<_>>().len() as u32
}
