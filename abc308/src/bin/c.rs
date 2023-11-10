use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [(f64, f64); n],
    }
    let mut r = vec![(0_usize, 0_f64); n];
    for i in 0..n {
        r[i] = (i+1, p[i].0 / (p[i].0 + p[i].1));
    }
    r.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    // r.sort_by(|a, b| (-a.1).partial_cmp(&(-b.1)).unwrap());
    for i in 0..n {
        print!("{} ", r[i].0);
    }

}
