fn main() {
    let input = String::from_utf8(std::fs::read("./src/input.txt").unwrap()).unwrap();
    let seeds = parse_seeds(&input);
    let maps = parse_maps(&input);

    let mut lowest_location = u64::MAX;
    for seed in seeds {
        let mut input = seed;
        for i in 0..7 {
            input = apply_map(input, &maps, i);
        }
        lowest_location = lowest_location.min(input);
    }
    println!("{lowest_location}");
}

fn parse_seeds(input: &str) -> [u64; 20] {
    let first_line = input.lines().collect::<Vec<_>>()[0];
    let numbers_string = first_line.split_once(": ").unwrap().1;
    let numbers = numbers_string
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let numbers_array: [u64; 20] = numbers.try_into().unwrap();
    numbers_array
}

type Map = Vec<[u64; 3]>;

fn parse_maps(input: &str) -> [Map; 7] {
    let map_strings = &input.split("\n\r\n").collect::<Vec<_>>()[1..];

    let mut parsed_maps: [Map; 7] = [vec![], vec![], vec![], vec![], vec![], vec![], vec![]];

    for (i, map) in map_strings.iter().enumerate() {
        parsed_maps[i] = parse_single_map(map);
    }

    parsed_maps
}

fn parse_single_map(map: &str) -> Map {
    let lines = &map.lines().collect::<Vec<_>>()[1..];
    let map_data = lines
        .iter()
        .map(|line| {
            let numbers = line
                .split(" ")
                .map(|str| {
                    return str.split_whitespace().collect::<Vec<&str>>()[0]
                        .parse::<u64>()
                        .unwrap();
                })
                .collect::<Vec<u64>>();
            return numbers.try_into().unwrap();
        })
        .collect::<Vec<_>>();
    map_data
}

fn translate_input(translation: [u64; 3], item: u64) -> Option<u64> {
    if translation[1] <= item && item < translation[1] + translation[2] {
        return Some(item - translation[1] + translation[0]);
    }
    return None;
}

fn apply_map(input: u64, maps: &[Map; 7], index: usize) -> u64 {
    for translation in &maps[index] {
        if let Some(translated_input) = translate_input(*translation, input) {
            return translated_input;
        }
    }
    return input;
}
