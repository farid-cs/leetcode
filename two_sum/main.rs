fn two_sum(nums: &[i64], target: i64) -> (usize, usize) {
    for i in 0..nums.len()-1 {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == target {
                return (i, j);
            }
        }
    }

    unreachable!("Exactly one solution assumed");
}

fn main() {
    let nums = [10, 20, 30, 70];
    let target = 50;

    let pair = two_sum(&nums, target);
    assert_eq!(pair, (1, 2));
}
