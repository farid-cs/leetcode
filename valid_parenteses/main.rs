fn is_valid(s: String) -> bool {
    let mut chars: Vec<char> = vec![];

    for paren in s.chars() {
        match paren {
            '(' | '[' | '{' => chars.push(paren),
            _ => {
                let Some(opening) = chars.pop() else {
                    return false;
                };
                match (opening, paren) {
                    ('(', ')') | ('{', '}') | ('[', ']') => continue,
                    _ => return false,
                }
            },
        }
    }

    if chars.is_empty() {
        return true;
    }

    false
}

fn main() {
    assert!(is_valid("{}(())[]".to_string()));
}
