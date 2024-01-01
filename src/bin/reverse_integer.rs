pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut is_negative = false;
        let mut x = x;
        if x == 0 {
            return 0;
        }
        if x < 0 {
            is_negative = true;
            match x.checked_abs() {
                Some(r) => x = r,
                None => return 0,
            }
        }
        let mut rev: i32 = 0;
        while x > 0 {
            let rem = x % 10;
            if let Some(r) = rev.checked_mul(10).and_then(|r| r.checked_add(rem)) {
                rev = r;
            } else {
                return 0;
            }
            x /= 10;
        }
        if is_negative {
            // check overflow
            match rev.checked_neg() {
                Some(r) => rev = r,
                None => return 0,
            }
        }
        rev
    }
}

fn main() {
    let x = 2147;
    println!("x: {}", x);
    let result = Solution::reverse(x);
    println!("{:?}", result);
}
