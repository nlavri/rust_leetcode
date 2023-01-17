use std::vec;

struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // V1
        if nums.len() == 0 {
            return 0;
        }

        let mut l_index = 0usize;
        let mut r_index = nums.len() - 1;
        while l_index <= r_index {
            if nums[l_index] == val {
                if r_index == 0 {
                    break;
                }
                nums[l_index] = nums[r_index];
                r_index -= 1;
            } else {
                l_index += 1;
            }
        }
        l_index as i32

        // V2
        // let mut i = 0;
        // for j in 0..nums.len() {
        //     if nums[j] != val {
        //         nums[i] = nums[j];
        //         i += 1;
        //     }
        // }
        // return i as i32;

        // V3
        // let mut i = 0;
        // let mut n = nums.len();
        // while i < n {
        //     if nums[i] == val {
        //         nums[i] = nums[n - 1];
        //         n -= 1;
        //     } else {
        //         i += 1;
        //     }
        // }
        // n as i32
    }
}

fn main() {
    let mut input = vec![2];
    let result = Solution::remove_element(&mut input, 3);
    println!("{result:?}");

    assert_eq!(1, result);
    assert_eq!(vec![2], input);

    let mut input = vec![3, 3];
    let result = Solution::remove_element(&mut input, 3);
    println!("{result:?}");

    assert_eq!(0, result);
    assert_eq!(vec![3, 3], input);

    let mut input = vec![3, 2, 2, 3];
    let result = Solution::remove_element(&mut input, 3);
    println!("{result:?}");

    assert_eq!(2, result);
    assert_eq!(vec![2, 2, 2, 3], input);

    let mut input = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let result = Solution::remove_element(&mut input, 2);
    println!("{result:?}");

    assert_eq!(5, result);
    assert_eq!(vec![0, 1, 4, 0, 3, 0, 4, 2], input);
}
