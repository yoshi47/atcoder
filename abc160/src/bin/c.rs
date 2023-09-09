use proconio::input;

fn main() {
    input! {
        k: i64,
        n: usize,
        a: [i64; n],
    }

    let mut tmp = 0;
    let mut v = vec![];
    for (i, aa) in a.iter().enumerate() {
        if i == 0 {
            v.push(k - a.last().unwrap() + a[i]);
        } else {
            v.push(a[i] - a[i-1]);
        }
    }
    let ms = v.iter().max().unwrap();
    println!("{}", v.iter().sum::<i64>() - ms);
}
