fn main() {
    let input = String::from_utf8(std::fs::read("./src/input.txt").unwrap()).unwrap();
    let symbol_positions = get_symbols(&input);

    //println!("{symbol_positions:?}");
    let mut sum = 0;

    for position in symbol_positions {
        sum += sum_around_symbol(&input, position);
    }
    println!("{sum}");
}

fn get_symbols(input: &str) -> Vec<(usize, usize)> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut positions = vec![];
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.char_indices() {
            if !char.is_digit(10) && char != '.' {
                positions.push((x, y));
            }
        }
    }
    positions
}

fn sum_around_symbol(input: &str, position: (usize, usize)) -> i32 {
    let lines = input.lines().collect::<Vec<_>>();
    let mut output = 0;
    for y in position.1.saturating_sub(1)..=position.1 + 1 {
        let line = lines[y];
        let chars = line.chars().collect::<Vec<_>>();
        let mut skip = false;
        for x in position.0.saturating_sub(1)..=position.0 + 1 {
            let char = chars[x];
            // println!("origin: {position:?}, ({x},{y}), {char:?}");
            if char.is_digit(10){
                if !skip {
                    output += get_full_number(&chars, x);
                }
                skip = true;
            } else {
                skip = false;
            }
        }
    }
    output
}

fn get_full_number(chars: &Vec<char>, index: usize) -> i32 {
    let mut line = chars.clone();
    let (left, right) = line.split_at_mut(index);
    left.reverse();
    let mut number_digits = vec![];
    // println!("{left:?} | {right:?}");

    for char in left {
        if let Some(digit) = char.to_digit(10) {
            number_digits.push(digit.to_string());
        } else {
            break;
        }
    }

    number_digits.reverse();

    for char in right {
        if let Some(digit) = char.to_digit(10) {
            number_digits.push(digit.to_string());
        } else {
            break;
        }
    }

    let number = number_digits.concat().parse::<i32>().unwrap();
    number
}
