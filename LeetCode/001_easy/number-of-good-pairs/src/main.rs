fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    nums.iter()
        .fold([0; 101], |mut acc, &x| {
            acc[x as usize] += 1;
            acc
        })
        .iter()
        .filter(|&&x| x > 1)
        .map(|&x| x * (x - 1) / 2)
        .sum()
}

fn main() {
    let v = vec![1,2,3,1,1,3];
    assert_eq!(num_identical_pairs(v), 4);
}
