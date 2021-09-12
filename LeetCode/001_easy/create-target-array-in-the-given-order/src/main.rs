fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut v = Vec::with_capacity(nums.len());

    nums.iter()
        .zip(index.iter())
        .for_each(|(x, ind)| v.insert(*ind as usize, *x));
    v
}

fn main() {
    let nums = vec![0, 1, 2, 3, 4];
    let index = vec![0, 1, 2, 2, 1];
    assert_eq!(create_target_array(nums, index), vec![0, 4, 1, 3, 2]);
}
