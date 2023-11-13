use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut a = HashMap::new();
    for _i in 0..m {
        input!{
            k: usize,
            x: [usize; k],
        }
        for j in 0..k-1 {
            for k in j+1..k {
                let count = a.entry((x[j], x[k])).or_insert(0);
                *count += 1;
            }
        }
    }
    // println!("{}, {}", a.keys().len(), n * (n - 1) / 2);
    if a.keys().len() == n * (n - 1) / 2 {
        println!("Yes");
    } else {
        println!("No");
    }


}
