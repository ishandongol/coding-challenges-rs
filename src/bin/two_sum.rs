use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut numbers_hash_map = HashMap::<i32, i32>::new();
        for (index, num) in numbers.iter().enumerate() {
            let complement = target - num;
            if numbers_hash_map.contains_key(&complement) {
                return vec![
                    numbers_hash_map.get(&complement).unwrap().to_owned(),
                    index as i32,
                ];
            } else {
                numbers_hash_map.insert(*num, index as i32);
            }
        }
        vec![]
    }
}
fn main() {
    let numbers = vec![2, 7, 11, 15];
    let target = 18;
    println!("{:?}", Solution::two_sum(numbers, target));
}
