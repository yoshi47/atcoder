use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        qq: [(usize, usize); q],
    }
    let mut p = vec![0_usize; n];
    for i in 0..n-1 {
        if s[i] == s[i+1] {
            p[i+1] = p[i] + 1;
        } else {
            p[i+1] = p[i];
        }

    }
    for (l, r) in qq {
        // println!("{:?}", p);
        let ans = p[r-1] - p[l-1];
        println!("{ans}");
    }
}
