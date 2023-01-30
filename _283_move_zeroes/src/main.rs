struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let num_len = nums.len();
        if num_len == 1 {
            return;
        }

        let mut j = 0usize;
        let mut i = 0usize;

        while i < num_len {
            if nums[i] == 0 {
                j = i;
                i += 1;
                break;
            }
            i += 1;
        }

        while i < num_len {
            if nums[j] == 0 && nums[i] != 0 {
                nums.swap(i, j);
                j += 1;
            }
            i += 1;
        }
    }
}

fn main() {
    let mut input = vec![0, 1, 4, 3, 12];
    let result = Solution::move_zeroes(&mut input);
    println!("{input:?}");
    assert_eq!(vec![1, 4, 3, 12, 0], input);

    let mut input = vec![0, 1, 0, 3, 12];
    let result = Solution::move_zeroes(&mut input);
    println!("{input:?}");
    assert_eq!(vec![1, 3, 12, 0, 0], input);

    let mut input = vec![1, 0, 0, 3, 12];
    let result = Solution::move_zeroes(&mut input);
    println!("{input:?}");
    assert_eq!(vec![1, 3, 12, 0, 0], input);

    let mut input = vec![0];
    let result = Solution::move_zeroes(&mut input);
    println!("{input:?}");
    assert_eq!(vec![0], input);
}
