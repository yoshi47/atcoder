use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: String,
    }
    let out = match x.as_str() {
        "Monday" => 5,
        "Tuesday" => 4,
        "Wednesday" => 3,
        "Thursday" => 2,
        _ => 1,
    };

    println!("{}", out);
}
