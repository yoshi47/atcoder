use proconio::input;

fn main() {
    input! {
        mut l: [i64],
    }
    l.sort();
    let mut count = 0;
    for i in 0..l.len() {
        for j in 0..i {
            for k in 0..j {
                if l[k] != l[j] && l[j] != l[i] && l[k] + l[j] > l[i] {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
