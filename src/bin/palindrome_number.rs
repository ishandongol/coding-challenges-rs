pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut y = x;
        if y < 0 {
            return false;
        }
        let mut rev: i32 = 0;
        while y > 0 {
            rev = (rev * 10) + (y % 10);
            y /= 10;
        }
        rev == x
    }
}

fn main() {
    let x = 1221;
    let result = Solution::is_palindrome(x);
    println!("{:?}", result);
}
