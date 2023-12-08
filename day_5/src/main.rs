use std::sync::Arc;

fn main() {
    let input = String::from_utf8(std::fs::read("./src/input.txt").unwrap()).unwrap();
    let seed_ranges = parse_seeds(&input);
    let maps = Arc::new(parse_maps(&input));

    let mut lowest_location = u64::MAX;
    let mut handles = vec![];
    for range in seed_ranges {
        let maps_clone = maps.clone();
        handles.push(std::thread::spawn(move || {
            println!("{range:?}");
            let mut lowest_local = u64::MAX;
            for seed in range[0]..range[1] {
                if seed % 1000000 == 0 {
                    println!("{:.3}%", ((seed - range[0]) as f64 / (range[1]-range[0]) as f64) * 100.0);
                }
                let mut input = seed;
                for i in 0..7 {
                    input = apply_map(input, &maps_clone, i);
                }
                lowest_local = lowest_local.min(input);
            }
            return lowest_local;
        }));
    }

    for handle in handles {
        let location = handle.join().unwrap();
        println!("{location}");
        lowest_location = lowest_location.min(location);
    }
    println!("{lowest_location}");
}

fn parse_seeds(input: &str) -> [[u64; 2]; 10] {
    let first_line = input.lines().collect::<Vec<_>>()[0];
    let numbers_string = first_line.split_once(": ").unwrap().1;
    let numbers: [u64; 20] = numbers_string
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let mut seed_ranges = [[0; 2]; 10];

    for i in 0..10 {
        let range_start = numbers[i * 2];
        let range_end = range_start + numbers[i * 2 + 1];
        seed_ranges[i] = [range_start, range_end];
    }

    seed_ranges
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
