struct Solution {}

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut i = 0usize;
        let mut j = nums.len() - 1;

        while i < j {
            let left_mod = nums[i] % 2;
            let right_mod = nums[j] % 2;
            if left_mod > right_mod {
                nums.swap(i, j);
            }

            if right_mod != 0 {
                j -= 1;
            }

            if left_mod == 0 {
                i += 1;
            }
        }

        nums
    }
}

fn main() {
    let input = vec![0, 1, 2];
    let result = Solution::sort_array_by_parity(input);
    println!("{result:?}");
    assert_eq!(vec![0, 2, 1], result);

    let input = vec![4, 1, 8];
    let result = Solution::sort_array_by_parity(input);
    println!("{result:?}");
    assert_eq!(vec![4, 8, 1], result);

    let input = vec![3, 1, 2, 4];
    let result = Solution::sort_array_by_parity(input);
    println!("{result:?}");
    assert_eq!(vec![4, 2, 1, 3], result);

    let input = vec![4, 2, 5, 7];
    let result = Solution::sort_array_by_parity(input);
    println!("{result:?}");
    assert_eq!(vec![4, 2, 5, 7], result);

    let input = vec![0];
    let result = Solution::sort_array_by_parity(input);
    println!("{result:?}");
    assert_eq!(vec![0], result);
}
