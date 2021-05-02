use proconio::input;

fn main() {
    input! {
        x: [f64],
    }

    let mut manhattan_distance = 0.;
    let mut euclid_distance = 0.;
    let mut chebyshev_distance = 0.;

    for i in x.iter() {
        
        manhattan_distance += i.abs();
    }

    for j in x.iter() {
        euclid_distance += j.powf(2.);
    }

    for k in x.iter() {
        chebyshev_distance = k.abs().max(chebyshev_distance);
    }

    println!("{}", manhattan_distance);
    println!("{}", euclid_distance.sqrt());
    println!("{}", chebyshev_distance);

}
