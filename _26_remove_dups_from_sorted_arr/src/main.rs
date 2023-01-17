use std::vec;

struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[i] != nums[j] {
                i += 1;
                nums[i] = nums[j];
            }
        }
        return (i + 1) as i32;
    }
}

fn main() {
    let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let result = Solution::remove_duplicates(&mut input);
    println!("{result:?}");

    assert_eq!(5, result);
    assert_eq!(vec![0, 1, 2, 3, 4, 2, 2, 3, 3, 4], input);

    let mut input = vec![2, 2, 2];
    let result = Solution::remove_duplicates(&mut input);
    println!("{result:?}");

    assert_eq!(1, result);
    assert_eq!(vec![2, 2, 2], input);

    let mut input = vec![1, 1, 2];
    let result = Solution::remove_duplicates(&mut input);
    println!("{result:?}");

    assert_eq!(2, result);
    assert_eq!(vec![1, 2, 2], input);

    let mut input = vec![1, 2, 3, 4, 5];
    let result = Solution::remove_duplicates(&mut input);
    println!("{result:?}");

    assert_eq!(5, result);
    assert_eq!(vec![1, 2, 3, 4, 5], input);
}
