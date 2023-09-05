use proconio::input;

fn main() {
    input! {
        n: usize,
        d: f64,
        v: [(f64, f64); n],
    }
    let mut ans = 0;
    for (p, q) in v {
        if (p * p + q * q).sqrt() <= d {
            ans += 1;
        }
    }
    println!("{}", ans);
}
