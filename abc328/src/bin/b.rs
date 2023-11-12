use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }
    let mut ans = 0;

    for i in 0..n {
        for j in 1..d[i]+1 {
            let a: Vec<char> = format!("{}{}", i+1, j).chars().collect();
            if a.iter().all(|&x| x == a[0]) {
                ans += 1;
            }
        }

    }
    println!("{ans}");
}
