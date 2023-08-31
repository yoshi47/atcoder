use proconio::input;

fn main() {
    input! {
        k: i32,
        a: i32,
        b: i32,
    }
    let c = b / k * k;
    if a <= c {
        println!("OK");
    } else {
        println!("NG");
    }
}
