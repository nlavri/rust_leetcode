struct Solution {}

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        if num == 0 {
            return num;
        }

        //V1
        // let mut num = num;
        // let mut result = 0;
        // while num != 0 {
        //     if num % 2 == 0 {
        //         num /= 2;
        //     } else {
        //         num -= 1;
        //     }
        //     result += 1;
        // }
        // result

        //V2
        let mut num = num as u32;
        let mut result = 0;
        while num != 0 {
            if num & 0x01 == 1 {
                num = num & 0xFFFFFFFE;
            } else {
                num >>= 1;
            }
            result += 1;
        }
        result
    }
}

fn main() {
    let result = Solution::number_of_steps(14);
    assert_eq!(6, result);
    println!("{result:?}");

    let result = Solution::number_of_steps(68355);
    assert_eq!(22, result);
    println!("{result:?}");
}
