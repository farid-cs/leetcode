fn str_str(haystack: String, needle: String) -> i32 {
    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();

    for i in 0..=haystack.len()-needle.len() {
        for j in 0..needle.len() {
            if haystack[i+j] != needle[j] {
                break;
            }
            return i as i32;
        }
    }

   -1
}

fn main() {
    let index = str_str("Find me a string".to_string(), "me".to_string());
    println!("Index: {}", index);
}
