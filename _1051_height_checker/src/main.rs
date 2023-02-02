struct Solution {}

impl Solution {
    pub fn counting_sort(arr: &mut [u32], maxval: usize) {
        let mut occurences: Vec<usize> = vec![0; maxval + 1];

        for &data in arr.iter() {
            occurences[data as usize] += 1;
        }

        let mut i = 0;
        for (data, &number) in occurences.iter().enumerate() {
            for _ in 0..number {
                arr[i] = data as u32;
                i += 1;
            }
        }
    }

    pub fn height_checker(heights: Vec<i32>) -> i32 {
        // V1
        //let mut sorter_heights = heights.clone();
        //sorter_heights.sort();

        // return heights
        //     .iter()
        //     .zip(sorter_heights.iter())
        //     .map(|x| match x.0 == x.1 {
        //         true => 0,
        //         false => 1,
        //     })
        //     .sum();

        // V2
        // Count sort without builind sorted array.
        // Just compare heights with what should go in sorted array.
        let mut heighs_frequencies = vec![0; 101];
        let mut result = 0;

        for h in heights.iter() {
            heighs_frequencies[*h as usize] += 1;
        }

        let mut h_index = 0;
        for (value, &freq) in heighs_frequencies.iter().enumerate() {
            for _ in 0..freq {
                if heights[h_index] != value as i32 {
                    result += 1;
                }
                h_index += 1;
            }
        }
        result
    }
}

fn main() {
    let mut i = vec![1, 1, 4, 2, 1, 3];
    Solution::counting_sort(&mut i[..], 100);
    println!("{i:?}");

    let input = vec![1, 1, 4, 2, 1, 3];
    let result = Solution::height_checker(input);
    println!("{result:?}");
    assert_eq!(3, result);

    let input = vec![5, 1, 2, 3, 4];
    let result = Solution::height_checker(input);
    println!("{result:?}");
    assert_eq!(5, result);

    let input = vec![1, 2, 3, 4, 5];
    let result = Solution::height_checker(input);
    println!("{result:?}");
    assert_eq!(0, result);
}
