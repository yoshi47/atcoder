use proconio::input;

fn main() {
    input! {
        d: i32,
        t: i32,
        s: i32,
    }
    if d <= s * t {
        println!("Yes");
    } else {
        println!("No");
    }
}
