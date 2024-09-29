fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut len = 1;
    let mut current_unique = nums[0];

    for i in 1..nums.len() {
       if nums[i] != current_unique {
           current_unique = nums[i];
           nums[len] = current_unique;
           len += 1;
       }
    }

    len as i32
}

fn main() {
    let mut dup = vec![0, 0, 0, 0, 1, 1, 2, 2, 2, 3];
    let num = remove_duplicates(&mut dup);
    let answer = [0, 1, 2, 3];

    assert_eq!(dup[0..4], answer[0..4]);
    assert_eq!(num as usize, answer.len());
}
