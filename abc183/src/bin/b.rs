use proconio::input;

fn main() {
    input! {
        sx: f64,
        sy: f64,
        gx: f64,
        gy: f64,
    }
    println!("{}", (gx - sx) * (sy / (sy + gy)) + sx);
}
