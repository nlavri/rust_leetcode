pub fn is_valid(s: String) -> bool {

    let mut stack = String::new();

    for c in s.chars() {
        match c {
            '(' | '[' |  '{' =>{
                stack.push(c);
            },
            ')' | ']' |  '}' => {
                match (stack.pop(), c) {
                    (Some('(') , ')') => (),
                    (Some('[') , ']') => (),
                    (Some('{') , '}') => (),
                    _ => return false
                };
            },
            _ => return false
        }
    }
    
    return stack.len() == 0;
}

fn main() {
    let s = "()[]{}";
    println!("{} {}", s, is_valid(String::from(s)));
    assert_eq!(true, is_valid(String::from(s)));

    let s = "([]){}";
    println!("{} {}", s, is_valid(String::from(s)));
    assert_eq!(true, is_valid(String::from(s)));

    let s = "([)]{}";
    println!("{} {}", s, is_valid(String::from(s)));
    assert_eq!(false, is_valid(String::from(s)));

}
