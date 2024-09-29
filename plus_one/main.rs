fn to_i32(digits: &Vec<i32>) -> i32 {
    let mut num = 0;
    for digit in digits.iter() {
        num = num * 10 + digit;
    }

    num as i32
}

fn to_vec(mut num: i32) -> Vec<i32> {
    let mut digits: Vec<i32> = vec![];
    while num > 0 {
        digits.push(num%10);
        num /= 10;
    }
    digits.reverse();
    digits
}

fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let number = to_i32(&digits) + 1;

    to_vec(number)
}

fn main() {
    let digits = vec![1, 2, 4];
    println!("Result: {:?}", plus_one(digits));
}
