use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut y = vec![vec!['0'; w+2]; h+2];

    for i in 1..h+1 {
        for j in 1..w+1 {
            if s[i-1][j-1] == '#' {
                y[i][j] = '#';
                for k in 0..3 {
                    for l in 0..3 {
                        if !(y[i+k-1][j+l-1] == '#')  {
                            let n =  y[i+k-1][j+l-1] as u32 - 48 + 1;
                            let n = std::char::from_digit(n, 10).unwrap();
                            y[i+k-1][j+l-1] = n;
                        }
                    }
                }
            }
        }
    }
    for i in 1..h+1 {
        for j in 1..w+1 {
            print!("{}", y[i][j]);
        }
        println!();
    }
}
