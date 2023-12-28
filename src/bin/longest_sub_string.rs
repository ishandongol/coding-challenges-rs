use std::collections::HashMap;
pub struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::<char, usize>::new();
        let mut max = 0;
        let mut start_index = 0;
        for (index, character) in s.chars().enumerate() {
            if let Some(previous_index) = map.get(&character) {
                start_index = start_index.max(previous_index + 1);
            }
            max = max.max(index - start_index + 1);
            map.insert(character, index);
        }
        max as i32
    }
}

fn main() {
    let s = String::from("ishanishan");
    let result = Solution::length_of_longest_substring(s);
    println!("{:?}", result);
}
