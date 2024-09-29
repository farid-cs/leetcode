fn length_of_last_word(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut end = bytes.len();

    while bytes[end-1] == b' ' {
        end -= 1;
    }

    let mut start = end-1;

    while start != 0 && bytes[start-1] != b' ' {
        start -= 1;
    }

    (end - start) as i32
}

fn main() {
    let len = length_of_last_word("Hello my dear friend".into());
    println!("Length: {}", len);
}
