struct Solution {}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        //V1
        // (1..=n)
        //     .map(|x| match x {
        //         x35 if x35 % 3 == 0 && x35 % 5 == 0 => String::from("FizzBuzz"),
        //         x3 if x3 % 3 == 0 => String::from("Fizz"),
        //         x5 if x5 % 5 == 0 => String::from("Buzz"),
        //         _ => x.to_string(),
        //     })
        //     .collect()

        //V2
        (1..=n)
            .map(|x| match x {
                x3 if x3 % 3 == 0 => match x3 {
                    x35 if x35 % 5 == 0 => String::from("FizzBuzz"),
                    _ => String::from("Fizz"),
                },
                x5 if x5 % 5 == 0 => match x5 {
                    x53 if x53 % 3 == 0 => String::from("FizzBuzz"),
                    _ => String::from("Buzz"),
                },
                _ => x.to_string(),
            })
            .collect()
    }
}

fn main() {
    let result = Solution::fizz_buzz(15);
    assert_eq!(
        vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ],
        result
    );

    println!("{result:?}");
}
