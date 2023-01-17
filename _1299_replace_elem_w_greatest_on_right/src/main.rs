struct Solution {}

use std::mem;

impl Solution {
    fn max(arr: &[i32]) -> i32 {
        let arr_len = arr.len();
        if arr_len == 1 {
            return arr[0];
        }
        let mut max_val = arr[0];
        for ix in 1..arr_len {
            if arr[ix] > max_val {
                max_val = arr[ix];
            }
        }
        max_val
    }

    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let arr_len = arr.len();

        // V1
        // if arr_len == 1 {
        //     return vec![-1];
        // }
        // if arr_len == 2 {
        //     return vec![arr[1], -1];
        // }

        // let mut arr = arr;
        // for ix in 0..arr_len {
        //     if ix == arr_len - 1 {
        //         arr[ix] = -1;
        //     } else {
        //         arr[ix] = *arr[ix + 1..arr_len].iter().max().unwrap() // Solution::max(&arr[ix + 1..arr_len]);
        //     }
        // }

        // V2
        // let mut arr = arr;
        // let mut tmp = -1;
        // let mut max_right = -1;
        // for i in (0..arr_len).rev() {
        //     tmp = arr[i];
        //     arr[i] = max_right;
        //     max_right = max_right.max(tmp);
        // }

        // V3 fast and furious
        let mut arr = arr;
        let mut max_right = -1;
        for i in arr.iter_mut().rev() {
            max_right = max_right.max(mem::replace(i, max_right));
        }

        arr
    }
}

fn main() {
    let input = vec![17, 18, 5, 4, 6, 1];
    let result = Solution::replace_elements(input);
    println!("{result:?}");
    assert_eq!(vec![18, 6, 6, 6, 1, -1], result);

    let input = vec![400];
    let result = Solution::replace_elements(input);
    println!("{result:?}");
    assert_eq!(vec![-1], result);
}
