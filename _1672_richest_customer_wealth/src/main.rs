struct Solution {}

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        //V1
        return accounts.iter().map(|x| x.iter().sum()).max().unwrap_or(0);

        //V2
        // let mut max = 0;
        // for person in accounts {
        //     let mut inter_max = 0;
        //     for amount in person {
        //         inter_max += amount;
        //     }
        //     if inter_max > max {
        //         max = inter_max
        //     }
        // }
        // max
    }
}

fn main() {
    let result = Solution::maximum_wealth(vec![
        [2, 8, 7].to_vec(),
        [7, 1, 3].to_vec(),
        [1, 9, 5].to_vec(),
    ]);
    assert_eq!(17, result);
    println!("{result:?}");
}
