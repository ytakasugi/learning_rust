/*
Questions:
整数numsの配列と整数targetが与えられたとき、targetに加算されるような2つの数値のインデックスを返す。
各入力には正確に1つの解があると仮定し、同じ要素を2回使用することはできません。
答えはどのような順序でも返せます。
*/

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut v = Vec::new(); 

    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                v.push(i as i32);
                v.push(j as i32);
                //let mut v = vec![i as i32, j as i32];
            }
        }
    }
    v
}

fn main() {
    let v = vec![2,7,11,15];
    let t = 9;

    assert_eq!(two_sum(v, t), vec![0, 1]);
}
