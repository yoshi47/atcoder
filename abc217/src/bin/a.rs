use proconio::input;
use proconio::marker::Chars;
use std::cmp::Ordering;


fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let order = s.cmp(&t);
    match order {
        Ordering::Less => println!("Yes"),
        _ => println!("No"),
    }

}
