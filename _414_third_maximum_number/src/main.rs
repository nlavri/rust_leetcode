struct Solution {}

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        // V1
        let mut max = 0;
        let mut temp_max = 0;
        let mut flip = false;
        let mut first_one_after_flip = None;
        let mut index = 0;

        while index < len {
            let val = nums[index];
            match (val, flip) {
                (1, false) => {
                    temp_max += 1;
                }
                (1, true) => {
                    temp_max += 1;

                    if first_one_after_flip.is_none() {
                        first_one_after_flip = Some(index);
                    }
                }
                (0, false) => {
                    temp_max += 1;
                    flip = true;
                }
                (0, true) => {
                    max = max.max(temp_max);
                    temp_max = 0;
                    flip = false;
                    if let Some(ix) = first_one_after_flip {
                        index = ix;
                        first_one_after_flip = None;
                    }
                    continue;
                }
                _ => todo!(),
            }
            index += 1;
        }
        max = max.max(temp_max);
        max

        // V2
        // let mut left = 0;
        // let mut right = 0;
        // let mut zeros_cnt = 0;
        // let mut max = 0;
        // while right < len {
        //     if nums[right] == 0 {
        //         zeros_cnt += 1;
        //     }

        //     while zeros_cnt >= 2 {
        //         if nums[left] == 0 {
        //             zeros_cnt -= 1;
        //         }
        //         left += 1;
        //     }
        //     max = max.max(right - left + 1);
        //     right += 1;
        // }
        // return max as i32;
    }
}

fn main() {
    let input = vec![1, 0, 0, 1, 1, 0, 1];
    let result = Solution::find_max_consecutive_ones(input);
    println!("{result:?}");
    assert_eq!(4, result);

    let input = vec![1, 0, 1, 1, 0];
    let result = Solution::find_max_consecutive_ones(input);
    println!("{result:?}");
    assert_eq!(4, result);

    let input = vec![1, 0, 1, 1, 0, 1];
    let result = Solution::find_max_consecutive_ones(input);
    println!("{result:?}");
    assert_eq!(4, result);

    let input = vec![1, 0, 0, 0, 0];
    let result = Solution::find_max_consecutive_ones(input);
    println!("{result:?}");
    assert_eq!(2, result);

    let input = vec![1, 1, 1];
    let result = Solution::find_max_consecutive_ones(input);
    println!("{result:?}");
    assert_eq!(3, result);

    let input = vec![];
    let result = Solution::find_max_consecutive_ones(input);
    println!("{result:?}");
    assert_eq!(0, result);

    let input = vec![0, 0, 0];
    let result = Solution::find_max_consecutive_ones(input);
    println!("{result:?}");
    assert_eq!(1, result);

    let input = vec![0];
    let result = Solution::find_max_consecutive_ones(input);
    println!("{result:?}");
    assert_eq!(1, result);
    assert_eq!(1, result);

    let input = vec![0, 0, 1];
    let result = Solution::find_max_consecutive_ones(input);
    println!("{result:?}");
    assert_eq!(2, result);
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn main_test() {
        let input = vec![1, 0, 1, 1, 0];
        let result = Solution::find_max_consecutive_ones(input);
        println!("{result:?}");
        assert_eq!(34, result);

        let input = vec![1, 0, 1, 1, 0, 1];
        let result = Solution::find_max_consecutive_ones(input);
        println!("{result:?}");
        assert_eq!(4, result);

        let input = vec![1, 2, 3, 4, 5];
        let result = Solution::find_max_consecutive_ones(input);
        println!("{result:?}");
        assert_eq!(0, result);
    }
}
