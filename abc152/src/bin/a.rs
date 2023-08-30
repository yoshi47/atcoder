use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
    }
    if n > m {
        println!("No");
    } else {
        println!("Yes")
    }
}
