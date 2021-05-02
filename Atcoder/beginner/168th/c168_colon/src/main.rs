use proconio::input;

fn main() {
    input!{
        // 時針
        a: f64,
        // 分針
        b: f64,
        // 時刻
        h: f64,
        m: f64,
    }
    
    // 分針の，12 時方向に対する角度（ラジアン）
    let minute_hand = m / 60. * 2. * std::f64::consts::PI;
    // 時針の，12 時方向に対する角度（ラジアン）
    let hour_hand = (h * 60. + m) / (12. * 60.) * 2. * std::f64::consts::PI;
    // 分針と時針のなす角度
    let angle = minute_hand - hour_hand;
    // 分針の先と時針の先の距離の2乗(余弦定理)
    // cosx = b^2 + c^2 * a^2 / 2bc
    let doubled = a * a + b * b - 2. * a * b * angle.cos();
    
    println!("{}", doubled.sqrt());
}
