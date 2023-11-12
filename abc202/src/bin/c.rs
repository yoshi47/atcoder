// use proconio::input;

// fn main() {
//     input! {
//         n: usize,
//         a: [usize; n],
//         b: [usize; n],
//         c: [usize; n],
//     }
//     let mut ans = 0;
//     let mut bc = vec![0_usize; n];
//     for i in 0..n {
//         bc[i] = b[c[i]-1];
//     }
//     for i in 0..n {
//         ans += bc.iter().filter(|&&x| x == a[i]).count();
//     }

//     // for i in 0..n {
//     //     for j in 0..n {
//     //         if a[i] == b[c[j]-1] {
//     //             ans += 1;
//     //         }
//     //     }
//     // }
//     println!("{ans}");
// }


use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }

    let mut ans = 0;
    let mut count_map: HashMap<usize, usize> = HashMap::new();

    for i in 0..n {
        let key = b[c[i] - 1];
        let count = count_map.entry(key).or_insert(0);
        *count += 1;
    }

    for i in 0..n {
        ans += count_map.get(&a[i]).cloned().unwrap_or(0);
    }

    println!("{}", ans);
}