use itertools::Itertools;
use proconio::input;

// fn main() {
//     input! {
//         n: usize,
//         p: [(f64, f64); n],
//     }
//     let mut r = vec![(0_usize, 0_f64); n];
//     for i in 0..n {
//         r[i] = (i+1, p[i].0 / (p[i].0 + p[i].1));
//     }
//     r.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
//     // r.sort_by(|a, b| (-a.1).partial_cmp(&(-b.1)).unwrap());
//     for i in 0..n {
//         print!("{} ", r[i].0);
//     }

// }

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut p: Vec<(usize, &(usize, usize))> = ab.iter().enumerate().collect();
    p.sort_by(|(_, a), (_, b)| {
        (b.0 * (a.0 + a.1)).partial_cmp(&(a.0 * (b.0 + b.1))).unwrap()
    });

    println!("{}", p.iter().map(|(i, _)| (i+1)).join(" "));
}
