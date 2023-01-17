struct Solution {}

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let arr_len = arr.len();
        if arr_len < 3 {
            return false;
        }

        // V1
        // let mut increasing = false;
        // let mut decreasing = false;
        // for index in 1..arr_len {
        //     if arr[index] > arr[index - 1] {
        //         if decreasing {
        //             return false;
        //         }
        //         increasing = true;
        //         continue;
        //     } else if arr[index] < arr[index - 1] {
        //         decreasing = true;
        //         continue;
        //     }
        //     return false;
        // }

        // increasing && decreasing

        // V2 patter matching
        let mut increasing = false;
        let mut decreasing = false;
        for index in 1..arr_len {
            let diff = arr[index] - arr[index - 1];
            match diff {
                0 => return false,
                1_i32..=i32::MAX => {
                    if decreasing {
                        return false;
                    }
                    increasing = true;
                }
                i32::MIN..=-1 => decreasing = true,
            }
        }
        increasing && decreasing

        // V3 ensure walk up and down
        // let mut i = 0;
        // let arr_len = arr.len();

        // while i < arr_len - 1 && arr[i] < arr[i + 1] {
        //     i += 1;
        // }

        // if i == arr_len - 1 || i == 0 {
        //     return false;
        // }

        // while i < arr_len - 1 && arr[i] > arr[i + 1] {
        //     i += 1;
        // }

        // i == arr_len - 1
    }
}

fn main() {
    let input = vec![1, 2, 3, 4];
    let result = Solution::valid_mountain_array(input);
    println!("{result:?}");
    assert_eq!(false, result);

    let input = vec![4, 3, 2, 1];
    let result = Solution::valid_mountain_array(input);
    println!("{result:?}");
    assert_eq!(false, result);

    let input = vec![0, 3, 2, 1, 5];
    let result = Solution::valid_mountain_array(input);
    println!("{result:?}");
    assert_eq!(false, result);

    let input = vec![0, 3, 2, 1];
    let result = Solution::valid_mountain_array(input);
    println!("{result:?}");
    assert_eq!(true, result);

    let input = vec![3, 5, 5];
    let result = Solution::valid_mountain_array(input);
    println!("{result:?}");
    assert_eq!(false, result);

    let input = vec![-20, 8, -6, -14, 0, -19, 14, 4];
    let result = Solution::valid_mountain_array(input);
    println!("{result:?}");
    assert_eq!(false, result);

    let input = vec![0, 0];
    let result = Solution::valid_mountain_array(input);
    println!("{result:?}");
    assert_eq!(false, result);
}
