struct Solution {}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        // V1
        // let mut result = Vec::<i32>::new();

        // for (ix, val) in nums.iter().enumerate() {
        //     match ix {
        //         0 => result.push(*val),
        //         _ => result.push(*val + result[ix - 1]),
        //     }
        // }

        //return result;

        //V2
        // let mut accum = 0;
        // return nums
        //     .iter()
        //     .map(|x| {
        //         accum = accum + x;
        //         return accum;
        //     })
        //     .collect();

        //V3
        // return nums
        //     .iter()
        //     .scan(0, |accum, x| {
        //         *accum = *accum + x;
        //         return Some(*accum);
        //     })
        //     .collect();

        //V4
        let mut nums = nums;
        for i in 1..nums.len() {
            nums[i] = nums[i - 1] + nums[i];
        }
        return nums;
    }
}

fn main() {
    let sln = Solution {};
    let result = Solution::running_sum(vec![1, 1, 1, 1]);
    println!("{result:?}");
}
