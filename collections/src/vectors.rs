/*
Given a list of integers, use a vector and return the median (when sorted, the 
value in the middle position) and mode (the value that occurs most often; a 
hash map will be helpful here) of the list.
*/
use rand::Rng;
use std::collections::HashMap;
use std::hash::Hash;
use num::Integer;

struct Range {
    pub min: i32,
    pub max: i32,
}

fn random_list(n: i32, r: &Range) -> Vec<i32> {
    
    let mut rng = rand::thread_rng();
    let numbers: Vec<i32> = (0..n).map(|_| rng.gen_range(r.min..=r.max)).collect();
    return numbers;
}

fn median<T>(list: &Vec<T>) -> Option<T>
    where
        T: Integer + Copy + From<i32>
{
    if list.is_empty() {
        return None;
    }
    let mut list: Vec<T> = list.clone();
    list.sort();
    let middle_index = list.len() / 2;
    // even
    if list.len() % 2 == 0 {
        return Some((list[middle_index] + list[middle_index - 1]) / T::from(2));
    }
    // odd
    else {
        return Some(list[middle_index]);
    }
}

fn mode<T>(list: &Vec<T>) -> Option<Vec<T>>
    where
        T: Eq + Hash + Copy
{
    if list.is_empty() {
        return None;
    }
    let mut frequency_map: HashMap<T, usize> = HashMap::new();
    // count the frequency of each number
    for value in list {
        *frequency_map.entry(value.clone()).or_default() += 1;
    }
    // Find the highest frequency
    let max_frequency = frequency_map.values().cloned().max().unwrap_or(0);
    // Collect all the numbers with the highest frequency
    return  Some(frequency_map
        .into_iter()
        .filter(|(_, frequency)| *frequency == max_frequency)
        .map(|(number, _)| number)
        .collect())
}

pub fn example() {
    let amount = 100;
    let range = Range{min: 0, max: 20};
    let list = random_list(amount, &range);
    println!(
        "Here is a list of {amount} random numbers ranging from [{}, {}]:\n{:?}",
        range.min, range.max, &list);

    let median = median(&list).unwrap();
    println!("The median of the list above is {median}");
    
    let mode = mode(&list).unwrap();
    if mode.len() > 1 {
        println!("The modes of the list above are {:?}", &mode);
    } else {
        println!("The mode of the list above is {}", mode[0]);
    }
}