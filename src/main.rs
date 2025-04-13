fn main() {
    let arr = [
        "(){}[]".to_string(),
        "([{}])".to_string(),
        "(}".to_string(),
        "[(])".to_string(),
        "[({})](]".to_string(),
    ];

    for str in arr {
        println!("{} => {}", str, valid_braces(&str))
    }
}

fn valid_braces(s: &str) -> bool {
    let mut stack = Vec::new();

    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => return false,
        }
    }

    stack.is_empty()
}
