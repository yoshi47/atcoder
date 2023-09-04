use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [(i64, i64); n],
    }
    let mut sum_num: i64 = 0;
    for &i in &v {
        sum_num += (i.0 + i.1) * (i.1 - i.0 + 1) / 2;
    }
    println!("{}", sum_num);
}
