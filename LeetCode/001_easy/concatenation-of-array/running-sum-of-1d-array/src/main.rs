fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    nums
        .into_iter()
        .map(|i| {sum += i; sum})
        .collect::<Vec<_>>()
}

fn main() {
    let v = vec![1,2,3,4];
    assert_eq!(running_sum(v), vec![1, 3, 6, 10]);
}
