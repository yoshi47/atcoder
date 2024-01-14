use proconio::input;

fn main() {
    input!{
        a: [i128],
    };
    if a.contains(&0) {
        println!("0");
        return
    }

    let mut acc = 1;
    for v in a {
        acc *= v;
        if acc > 10_i128.pow(18) {
            println!("-1");
            return
        }
    }
    println!("{acc}");
}
