use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    // let mut ans = 0;
    // if n % 2 == 0 {
    //     for i in (1..n).step_by(2) {
    //         ans += i % i+1;
    //         ans += (i + 1) % i;
    //     }
    // } else {
    //     let mut a = vec![0_usize; n];
    //     ans += 1 % 3;
    //     ans += 2 % 1;
    //     ans += 3 % 2;
    //     a[0] += 3;
    //     a[1] += 2;
    //     a[2] += 1;
    //     for i in (4..n).step_by(2) {
    //         ans += i % i+1;
    //         ans += (i + 1) % i;
    //         a[i-1] += i+1;
    //         a[i] += i;
    //     }
    //     println!("{:?}", a);
    // }
    // println!("{ans}");
    let mut a = 0;
    for i in 1..n {
        a += i;
    }
    println!("{a}");
    // let a: Vec<usize> = (1..n).collect();
    // println!("{}", a.iter().sum::<usize>());
}
