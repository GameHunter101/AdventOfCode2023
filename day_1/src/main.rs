const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBER_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = String::from_utf8(std::fs::read("./src/input.txt").unwrap()).unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let mut result = 0;
    for line in lines {
        result += parse_line(line);
    }
    println!("{result}");
}

fn parse_line(line: &str) -> i32 {
    let mut calibration_string = "".to_string();
    let mut first_word_index = i32::MAX;
    let mut first_word = "".to_string();
    for number in NUMBER_WORDS {
        let index = line.split(number).collect::<Vec<_>>()[0].len() as i32;
        if index < first_word_index {
            first_word_index = index;
            first_word = number.to_string();
        }
    }

    let mut last_word_index = i32::MAX;
    let mut last_word = "".to_string();
    for number in NUMBER_WORDS {
        let index = line.split(number).collect::<Vec<_>>().last().unwrap().len() as i32;
        if index < last_word_index {
            last_word_index = index;
            last_word = number.to_string();
        }
    }

    let mut first_num_index = i32::MAX;
    let mut first_num = "".to_string();
    for (i, char) in line.chars().enumerate() {
        if !ALPHABET.contains(&char.to_lowercase().to_string()) {
            first_num_index = i as i32;
            first_num = char.to_string();
            break;
        }
    }

    let mut last_num_index = i32::MAX;
    let mut last_num = "".to_string();
    for (i, char) in line.chars().rev().enumerate() {
        if !ALPHABET.contains(&char.to_lowercase().to_string()) {
            last_num_index = i as i32;
            last_num = char.to_string();
            break;
        }
    }

    if first_num_index < first_word_index {
        calibration_string += &first_num;
    } else {
        calibration_string +=
            &(NUMBER_WORDS.iter().position(|&e| e == first_word).unwrap() + 1).to_string();
    }

    if last_num_index < last_word_index {
        calibration_string += &last_num;
    } else {
        calibration_string +=
            &(NUMBER_WORDS.iter().position(|&e| e == last_word).unwrap() + 1).to_string();
    }

    let calibration_value = calibration_string.parse().unwrap();
    calibration_value
}
