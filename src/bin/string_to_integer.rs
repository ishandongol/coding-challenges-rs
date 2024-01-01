pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut is_negative = false;
        let s = s.trim().as_bytes();
        let mut start_index = 0;
        let mut result = String::new();
        if s.is_empty() {
            return 0;
        }
        if s[0] == b'-' {
            is_negative = true;
            start_index = 1;
        } else if s[0] == b'+' {
            start_index = 1;
        }
        for index in start_index..s.len() {
            if !s[index].is_ascii_digit() {
                break;
            }
            result.push(char::from(s[index]));
        }
        if is_negative {
            result.insert(0, '-');
        }
        match result.parse::<i32>() {
            Ok(r) => r,
            Err(error) => match error.kind() {
                std::num::IntErrorKind::PosOverflow => i32::MAX,
                std::num::IntErrorKind::NegOverflow => i32::MIN,
                _ => return 0,
            },
        }
    }
}

fn main() {
    let s = String::from("2147483647s");
    let result = Solution::my_atoi(s);
    println!("{:?}", result);
}
