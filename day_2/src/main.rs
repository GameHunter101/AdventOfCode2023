#![allow(unused)]

const PASSING: CubeCounts = CubeCounts {
    red: 12,
    blue: 14,
    green: 13,
};

fn main() {
    let input = String::from_utf8(std::fs::read("./src/input.txt").unwrap()).unwrap();
    let lines = input.lines().collect::<Vec<_>>();

    let mut sum = 0;

    for line in lines {
        sum += parse_line(line.to_string());
    }
    println!("{sum}");
}

fn parse_line(line: String) -> i32 {
    let (meta, data) = line.split_once(": ").unwrap();
    let game_id: i32 = meta.split_at(5).1.parse().unwrap();
    parse_data(data)
}

fn parse_data(data: &str) -> i32 {
    let cube_sets = data.split("; ").collect::<Vec<_>>();
    let mut accumulated_counts = CubeCounts::default();
    for set in cube_sets {
        let parsed_set = parse_set(set);
        accumulated_counts = accumulated_counts.max(&parsed_set);
    }
    accumulated_counts.power()
}

#[derive(Debug)]
struct CubeCounts {
    red: i32,
    blue: i32,
    green: i32,
}

impl CubeCounts {
    pub fn max(&self, other: &Self) -> Self {
        CubeCounts {
            red: self.red.max(other.red),
            blue: self.blue.max(other.blue),
            green: self.green.max(other.green),
        }
    }

    pub fn passes(&self) -> bool {
        self.red <= PASSING.red && self.blue <= PASSING.blue && self.green <= PASSING.green
    }

    pub fn power(&self) -> i32 {
        self.red * self.blue * self.green
    }
}

impl Default for CubeCounts {
    fn default() -> Self {
        Self {
            red: 0,
            blue: 0,
            green: 0,
        }
    }
}

fn parse_set(set: &str) -> CubeCounts {
    let no_whitespaces = set.split_whitespace().collect::<String>();
    let mut last_char_was_digit = false;
    let mut accumulated_number = String::new();
    let mut set_cubes = CubeCounts::default();

    for char in no_whitespaces.chars() {
        match char.to_digit(10) {
            Some(_) => {
                last_char_was_digit = true;
                accumulated_number.push(char);
            }
            None => {
                if last_char_was_digit {
                    last_char_was_digit = false;
                    match char {
                        'r' => set_cubes.red = accumulated_number.parse().unwrap(),
                        'b' => set_cubes.blue = accumulated_number.parse().unwrap(),
                        'g' => set_cubes.green = accumulated_number.parse().unwrap(),
                        _ => {}
                    }
                    accumulated_number = String::new();
                }
            }
        }
    }
    set_cubes
}
