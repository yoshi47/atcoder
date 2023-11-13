use proconio::input;

fn main() {
    input! {
        x: i64,
    }
    let (mut a, mut b) = (0_i64, -1_i64);
    loop {
        if a.pow(5) - b.pow(5) == x {
            println!("{} {}", a, b);
            break
        }
        a += 1;
        b -= 1;
    }
}
