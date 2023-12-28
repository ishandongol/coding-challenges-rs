pub struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(num_one: Vec<i32>, num_two: Vec<i32>) -> f64 {
        let mut i = 0;
        let mut j = 0;
        let mut sorted_array: Vec<i32> = vec![];
        let mut remaining_array: Vec<i32> = vec![];
        if num_one.len() == 0 {
            sorted_array.extend(num_two)
        } else if num_two.len() == 0 {
            sorted_array.extend(num_one)
        } else {
            loop {
                if num_one[i] < num_two[j] {
                    sorted_array.push(num_one[i]);
                    i += 1;
                    if i == num_one.len() {
                        remaining_array.extend_from_slice(&num_two[j..]);
                        break;
                    }
                } else {
                    sorted_array.push(num_two[j]);
                    j += 1;
                    if j == num_two.len() {
                        remaining_array.extend_from_slice(&num_one[i..]);
                        break;
                    }
                }
            }
        }
        sorted_array.extend(remaining_array);
        let length = sorted_array.len();
        if length % 2 == 0 {
            let mid = length / 2;
            (sorted_array[mid] + sorted_array[mid - 1]) as f64 / 2.0
        } else {
            sorted_array[length / 2] as f64
        }
    }
}

fn main() {
    let num_one = vec![];
    let num_two = vec![2, 10, 15];
    let result = Solution::find_median_sorted_arrays(num_one, num_two);
    println!("{:?}", result);
}
