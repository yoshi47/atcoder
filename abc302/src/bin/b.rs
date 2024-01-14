use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let dir = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    for i in 0..h {
        for j in 0..w {
            for &(dx, dy) in &dir {
                let mut ans = Vec::new();
                for k in 0..5 {
                    let x = i as isize + dx * k as isize;
                    let y = j as isize + dy * k as isize;

                    if x < 0 || x >= h as isize || y < 0 || y >= w as isize {
                        break;
                    }

                    let x = x as usize;
                    let y = y as usize;
                    ans.push(s[x][y]);
                }
                if ans == ['s', 'n', 'u', 'k', 'e'] {
                    for k in 0..5 {
                        let x = i as isize + dx * k as isize + 1;
                        let y = j as isize + dy * k as isize + 1;
                        println!("{} {}", x, y);
                    }
                    return;
                }
            }
        }
    }
}