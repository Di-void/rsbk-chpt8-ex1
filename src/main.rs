// Problem:
// Given a list of integers, use a vector and return the mean, median and mode of the list.

// mean - the average value
// median - when sorted, the value in the middle position
// mode - the value that occurs the most often (hash map will be helpful here)

use std::collections::HashMap;

fn main() {
    // list of integers
    let mut result = HashMap::new();
    let mut integers = vec![2, 3, 5, 1, 7, 10, 15, 10];
    let sum: i32 = integers.iter().sum();
    let mean: i32 = sum / integers.len() as i32;

    result.insert(String::from("mean"), mean);

    result.insert(String::from("mode"), *find_mode(&integers));

    result.insert(String::from("median"), find_median(&mut integers));

    println!("{:?}", result);
}

fn find_mode (list: &Vec<i32>) -> &i32 {
    let mut map = HashMap::new();
    let mut mode: &i32 = &list[0];
    for i in list.iter() {
        let count = map.entry(i).or_insert(0);
        *count += 1; // deref first to change value of entry
    }

    if let Some(n) = map.get(mode) {
        let mut count = n;
        for (key, value) in &map {
            if value > count {
                mode = key
            } else {
                count = value;
            }
        }

    };
    
    mode
}

fn find_median (list: &mut Vec<i32>) -> i32 {
    list.sort_unstable();
    
    let is_list_even = list.len() % 2 == 0;
    
    let midpoint = ((list.len() + 1) / 2) - 1;

    if is_list_even {
        (list[midpoint] + list[midpoint + 1]) / 2
    } else {
        list[midpoint]
    }
}