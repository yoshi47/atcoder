use proconio::input;

fn main() {
    input! {
        r: f64,
    }
    println!("{}", 2. * r * std::f64::consts::PI)
}
