fn is_palindrome(mut x: i64) -> bool {
    let original = x;
    let mut reversed = 0;

    if original < 0 {
        return false;
    }

    while x != 0 {
        reversed = reversed * 10 + x % 10;
        x /= 10;
    }

    original == reversed
}

fn main() {
    let x = 101;
    assert!(is_palindrome(x));
}
