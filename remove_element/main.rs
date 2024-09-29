fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut len = 0;

    for i in 0..nums.len() {
        if nums[i] != val {
            nums[len] = nums[i];
            len += 1;
        }
    }

    len as i32
}

fn main() {
    let mut sample = vec![0, 0, 0, 1, 0, 10, 20, 30, 40, 90, 0, 0, 20, 72];
    let len = remove_element(&mut sample, 0) as usize;
    println!("Remaining numbers: {:?}", &sample[..len]);
}
