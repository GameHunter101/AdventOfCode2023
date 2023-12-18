use std::{
    cmp::{Ordering, PartialEq},
    collections::HashSet,
    fmt::{Debug, Display},
};

use enum_index::EnumIndex;
use enum_index_derive::{EnumIndex, IndexEnum};

use crate::quicksort::sort;

mod quicksort;

static CARD_TYPES: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn main() {
    let input = String::from_utf8(std::fs::read("./src/input.txt").unwrap()).unwrap();
    let lines: [&str; 1000] = input.lines().collect::<Vec<&str>>().try_into().unwrap();

    let mut parsed_lines = Vec::with_capacity(1000);
    for line in lines {
        let parsed = parse(line);
        parsed_lines.push(parsed);
    }

    parsed_lines.sort();

    let mut sum = 0;
    for (i, hand) in parsed_lines.iter().enumerate() {
        let hand_val = (i as u32 + 1) * hand.bid;
        sum += hand_val;
    }
    println!("{sum}");
}

fn parse(line: &str) -> Hand {
    let (hand_str, bid_str) = line.split_once(" ").unwrap();
    let bid = bid_str.parse::<u32>().unwrap();
    let hand_chars: [char; 5] = hand_str.chars().collect::<Vec<_>>().try_into().unwrap();

    let hand_type = get_hand_type(hand_chars).unwrap();
    Hand::new(hand_chars, hand_type, bid)
}

fn get_hand_type(hand_chars: [char; 5]) -> Option<HandType> {
    let hand_set = HashSet::from(hand_chars);
    let joker_count = hand_chars.iter().count_occurances(&'J');
    let occurances_set = create_occurances_set(
        &hand_chars
            .iter()
            .cloned()
            .filter(|&c| c != 'J')
            .collect::<Vec<_>>(),
        hand_set,
    );
    let (counts, chars): (Vec<u32>, Vec<char>) = occurances_set.iter().cloned().unzip();

    let mut modified_hand_chars = hand_chars.clone();

    if joker_count > 0 {
        let most_frequent_char = chars[get_largest_index(&counts)];
        for (i, c) in hand_chars.iter().enumerate() {
            if c == &'J' {
                modified_hand_chars[i] = most_frequent_char;
            }
        }
    }

    let hand_set = HashSet::from(modified_hand_chars);
    let set_length = hand_set.len();
    let occurances_set = create_occurances_set(&modified_hand_chars, hand_set);
    let (counts, _): (Vec<u32>, Vec<char>) = occurances_set.iter().cloned().unzip();

    match set_length {
        5 => return Some(HandType::HighCard),
        4 => return Some(HandType::OnePair),
        3 => {
            if counts.contains(&2) {
                return Some(HandType::TwoPair);
            }
            return Some(HandType::ThreeOfAKind);
        }
        2 => {
            if counts.contains(&3) {
                return Some(HandType::FullHouse);
            }
            return Some(HandType::FourOfAKind);
        }
        1 => {
            return Some(HandType::FiveOfAKind);
        }
        _ => return None,
    }
}

fn get_largest_index(occurances_set: &Vec<u32>) -> usize {
    let mut index = 0;
    let mut highest_count = 0;
    for (i, count) in occurances_set.iter().enumerate() {
        if count > &highest_count {
            highest_count = *count;
            index = i;
        }
    }
    index
}

fn create_occurances_set(hand_chars: &[char], hand_set: HashSet<char>) -> HashSet<(u32, char)> {
    let occurances = hand_set
        .iter()
        .map(|i| (hand_chars.iter().count_occurances(i), *i))
        .collect::<Vec<(u32, char)>>();
    let occurances_set: HashSet<(u32, char)> = HashSet::from_iter(occurances.iter().cloned());

    occurances_set
}

#[derive(Debug, IndexEnum, EnumIndex, Clone, Copy, Ord, PartialEq, PartialOrd, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Clone, Copy, Eq, Ord)]
struct Hand {
    cards: [char; 5],
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    fn new(cards: [char; 5], hand_type: HandType, bid: u32) -> Self {
        Self {
            cards,
            hand_type,
            bid,
        }
    }

    fn compare_cards(&self, other: &Self) -> Ordering {
        for i in 0..5 {
            let self_card = self.cards[i];
            let other_card = other.cards[i];

            let self_card_index = CARD_TYPES.iter().position(|&c| c == self_card).unwrap();
            let other_card_index = CARD_TYPES.iter().position(|&c| c == other_card).unwrap();

            if self_card_index > other_card_index {
                return Ordering::Less;
            }
            if self_card_index < other_card_index {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.cards)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type.enum_index() == other.hand_type.enum_index() {
            if self.cards == other.cards {
                return true;
            }
        }
        return false;
    }
    fn ne(&self, other: &Self) -> bool {
        if self.hand_type.enum_index() == other.hand_type.enum_index() {
            if self.cards != other.cards {
                return true;
            }
        }
        return true;
    }
}

impl PartialOrd for Hand {
    fn lt(&self, other: &Self) -> bool {
        if self.partial_cmp(other).unwrap() == Ordering::Less {
            return true;
        }
        return false;
    }
    fn gt(&self, other: &Self) -> bool {
        if self.partial_cmp(other).unwrap() == Ordering::Greater {
            return true;
        }
        return false;
    }
    fn le(&self, other: &Self) -> bool {
        if self < other || self == other {
            return true;
        }
        return false;
    }
    fn ge(&self, other: &Self) -> bool {
        if self > other || self == other {
            return true;
        }
        return false;
    }
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type.enum_index() > other.hand_type.enum_index() {
            return Some(Ordering::Less);
        }
        if self.hand_type.enum_index() < other.hand_type.enum_index() {
            return Some(Ordering::Greater);
        }
        return Some(self.compare_cards(other));
    }
}

trait Occurances<T: PartialEq> {
    fn count_occurances(&self, item: &T) -> u32;
}

impl<T: PartialEq> Occurances<T> for std::slice::Iter<'_, T> {
    fn count_occurances(&self, item: &T) -> u32 {
        let mut count = 0;
        for i in self.clone().into_iter() {
            if i == item {
                count += 1;
            }
        }
        count
    }
}
