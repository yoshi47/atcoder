use proconio::input;

fn main() {
    input! {
        h: [isize; 3],
        w: [isize; 3],
    }
    let mut ans = 0;
    for a1 in 1..30 {
        for a2 in 1..30 {
            for b1 in 1..30 {
                for b2 in 1..30 {
                    let a3 = h[0] - a1 - a2;
                    let b3 = h[1] - b1 - b2;
                    let c1 = w[0] - a1 - b1;
                    let c2 = w[1] - a2 - b2;
                    let c3 = w[2] - a3 - b3;
                    if vec![a3, b3, c1, c2, c3].iter().min().unwrap() > &0 && c1 + c2 + c3 == h[2] {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{ans}");
}
