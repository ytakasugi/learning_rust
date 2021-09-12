fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut v = nums.clone();
    for i in nums.into_iter() {
        v.push(i);
    }
    v
}

fn main() {
    let nums = vec![1, 2, 3];
    assert_eq!(get_concatenation(nums), vec![1, 2, 1, 1, 2, 1]);
}
