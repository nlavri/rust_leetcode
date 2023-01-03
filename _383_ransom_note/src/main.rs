use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if magazine.len() < ransom_note.len() {
            return false;
        }

        let mut available_letters = HashMap::<char, i32>::new();
        for c in magazine.chars() {
            let e = available_letters.entry(c).or_insert(0);
            *e += 1;
        }

        // V1
        for c in ransom_note.chars() {
            let letter = available_letters.get_mut(&c);
            match letter {
                Some(l) => {
                    if *l == 0 {
                        return false;
                    }
                    *l -= 1;
                }
                None => return false,
            }
        }

        // //V2 entry API
        // for c in ransom_note.chars() {
        //     match available_letters.entry(c) {
        //         std::collections::hash_map::Entry::Occupied(mut x) => {
        //             let current_cnt = x.get_mut();
        //             if *current_cnt == 0 {
        //                 return false;
        //             }
        //             *current_cnt -= 1;
        //         }
        //         std::collections::hash_map::Entry::Vacant(_) => return false,
        //     }
        // }

        //V3 two hashmaps
        // let mut needed_letters = HashMap::<char, i32>::new();
        // for c in ransom_note.chars() {
        //     let e = needed_letters.entry(c).or_insert(0);
        //     *e += 1;
        // }

        // for (c, cnt) in needed_letters {
        //     match available_letters.get(&c) {
        //         Some(l) => {
        //             if *l < cnt {
        //                 return false;
        //             }
        //         }
        //         None => return false,
        //     }
        // }

        true
    }
}

fn main() {
    let ransom_note = String::from("a");
    let magazine = String::from("b");
    let result = Solution::can_construct(ransom_note, magazine);
    println!("{result:?}");
    assert_eq!(false, result);

    let ransom_note = String::from("aa");
    let magazine = String::from("aab");
    let result = Solution::can_construct(ransom_note, magazine);
    println!("{result:?}");
    assert_eq!(true, result);
}
