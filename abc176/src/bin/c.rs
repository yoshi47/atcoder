use proconio::input;

fn main() {
    input! {
        n : usize,
        a: [i64; n],
    }
    let mut ans = 0;
    let mut tmp = 0;
    for i in 0..n {
        if i == 0 {
            tmp = a[i];
            continue;
        }

        if a[i] < tmp {
            ans += tmp - a[i];
        } else {
            tmp = a[i];
        }
    }
    println!("{}", ans);
}
