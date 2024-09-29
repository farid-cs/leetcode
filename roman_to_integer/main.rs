fn roman_to_int (roman: String) -> i32 {
    let char_to_int = |ch| {
        match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => unreachable!("Unexpected character")
        }
    };

    let mut translated = roman
        .chars()
        .map(char_to_int);

    let mut sum = 0;
    let mut current = translated
        .next()
        .expect("Empty string");

    while let Some(next) = translated.next() {
        if current >= next {
            sum += current;
        } else {
            sum -= current;
        }

        current = next
    }

    sum + current
}

fn main() {
    roman_to_int("XV".into());
}
