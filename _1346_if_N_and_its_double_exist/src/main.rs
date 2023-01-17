use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    vec,
};

struct Solution {}

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        // V1
        // for i in 0..arr.len() {
        //     for j in i + 1..arr.len() {
        //         if arr[i] * 2 == arr[j] || (arr[i] % 2 == 0 && arr[i] / 2 == arr[j]) {
        //             return true;
        //         }
        //     }
        // }
        // false

        // V2
        let mut set = HashSet::<i32>::new();
        for i in arr.iter() {
            if let Some(_) = set.get(i) {
                return true;
            }

            set.insert(i * 2);
            if i % 2 == 0 {
                set.insert(*i / 2);
            }
        }

        false
    }
}

fn main() {
    let input = vec![-20, 8, -6, -14, 0, -19, 14, 4];
    let result = Solution::check_if_exist(input);
    println!("{result:?}");

    assert_eq!(true, result);

    let input = vec![0, 0];
    let result = Solution::check_if_exist(input);
    println!("{result:?}");

    assert_eq!(true, result);

    let input = vec![-2, 0, 10, -19, 4, 6, -8];
    let result = Solution::check_if_exist(input);
    println!("{result:?}");

    assert_eq!(false, result);

    let input = vec![10, 2, 5, 3];
    let result = Solution::check_if_exist(input);
    println!("{result:?}");

    assert_eq!(true, result);

    let input = vec![1, 3, 5, 7];
    let result = Solution::check_if_exist(input);
    println!("{result:?}");

    assert_eq!(false, result);
}
