extern crate stopwatch;
use stopwatch::{Stopwatch};

pub fn is_palindrome(x: i128) -> bool {
        if x < 0 {
            return false;
        }

        let strval = x.to_string();
        let reverse = strval.chars().rev().collect::<String>();
        return strval == reverse;
}

pub fn is_palindrome2(x: i128) -> bool {
    if x < 0 {
        return false;
    }

    let strval = x.to_string();
    let len = strval.len();
    let is_odd = len % 2;
    let half = len / 2;
    
    let result =  strval[..half] == strval[half + is_odd..].chars().rev().collect::<String>();
    return result;
}


pub fn is_palindrome3(x: i128) -> bool {
    if x < 0 {
        return false;
    }

    let mut rev = 0;
    let mut copy = x;
    
    while copy != 0 {
        let digit = copy % 10;
        rev = rev * 10 + digit;
        copy = copy / 10;
    }

    x == rev
}

fn main() {

    let mut x = 1234321;
    let mut ix = 0;
    while x != 0 {
        let z = x % 10;
        ix = ix * 10 + z;
        print!("{}", z);
        x = x / 10;
    }
    println!("-----------");
    println!("{}", ix);
    println!("-----------");

    println!("{}", is_palindrome3(123321));
    println!("{}", is_palindrome(1000000000000066600000000000001));
    println!("{}", is_palindrome(100000000000006600000000000001));
    println!("{}", is_palindrome2(1000000000000066600000000000001));
    println!("{}", is_palindrome2(100000000000006600000000000001));
    println!("{}", is_palindrome3(1000000000000066600000000000001));
    println!("{}", is_palindrome3(100000000000006600000000000001));


    let mut sw = Stopwatch::start_new();

    for i in 1..100000{
        let a: i128 = 1000000000000066600000000000001;
        is_palindrome(a);

        let a = -121;
        is_palindrome(a);

        let a = 10;
        is_palindrome(a);

        let a = 1234321;
        is_palindrome(a);
    }
    println!("ellapsed {}", sw.elapsed_ms());

    sw.restart();

    for i in 1..100000{
        let a: i128 = 1000000000000066600000000000001;
        is_palindrome2(a);

        let a = -121;
        is_palindrome2(a);

        let a = 10;
        is_palindrome2(a);

        let a = 1234321;
        is_palindrome2(a);
    }
    println!("ellapsed {}", sw.elapsed_ms());

    
    sw.restart();

    for i in 1..100000{
        let a: i128 = 1000000000000066600000000000001;
        is_palindrome3(a);

        let a = -121;
        is_palindrome3(a);

        let a = 10;
        is_palindrome3(a);

        let a = 1234321;
        is_palindrome3(a);
    }
    println!("ellapsed {}", sw.elapsed_ms());

}
