const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
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
    for char in line.chars() {
        if !ALPHABET.contains(&char.to_uppercase().to_string()) {
            calibration_string += &char.to_string();
            break;
        }
    }

    for char in line.chars().rev() {
        if !ALPHABET.contains(&char.to_uppercase().to_string()) {
            calibration_string += &char.to_string();
            break;
        }
    }
    calibration_string.parse().unwrap()
}
