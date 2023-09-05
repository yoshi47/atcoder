use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut p: [i32; n],
    }
    p.sort();
    let pp = &p[..k];
    println!("{}", pp.iter().sum::<i32>());
}
