use proconio::input;

fn main() {
    input! {
        n: i64,
        mut a: [i64; n],
    }
    let mut counter = 0;
    let mut all_even: bool = true;
    while all_even {
        all_even = a.iter().all(|&x| x % 2 == 0);
        if all_even {
            a = a.iter().map(|&x| x / 2).collect();
            counter += 1;
        } else {
            all_even = false;
        }
    }
    println!("{}", counter);
    }
