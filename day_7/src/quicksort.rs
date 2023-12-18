use std::fmt::Debug;

pub fn sort<'a, T: PartialOrd + PartialEq + Clone + Copy + Debug>(array: &'a [T]) -> Vec<T> {
    if array.len() == 1 {
        return array.to_vec();
    }
    if array.len() == 2 {
        if array[0] > array[1] {
            return vec![array[1], array[0]];
        }
        return array.to_vec();
    }
    if array.len() > 0 {

        let (first, last) = split_at_pivot(array, array[1]);
        let sorted_first: Vec<T> = sort(&first);
        let sorted_last: Vec<T> = sort(&last);
        return vec![sorted_first, sorted_last].concat();
    }
    return array.to_vec();
}
pub fn split_at_pivot<'a, T: PartialOrd + PartialEq + Clone + Copy + Debug>(array: &'a [T], pivot: T) -> (Vec<T>, Vec<T>) {
    let mut left_array: Vec<T> = vec![];
    let mut right_array: Vec<T> = vec![];
    
    for i in array {
        if i < &pivot {
            left_array.push(*i);
        } else {
            right_array.push(*i);
        }
    }
    (left_array,right_array)
}
