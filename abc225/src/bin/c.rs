use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[usize; m]; n],
    };
    for i in 0..n {
        if b[i][0..m-1].iter().all(|&x| x % 7 == 0) {
            println!("No");
            return;
        }
        // let row_s: usize = b[i].iter().sum();
        // let t_sum = m * (b[i][0] + b[i][m-1]) / 2;
        // if !(row_s == t_sum) {
        //     println!("No");
        //     return;
        // }

        if !(i == n-1) {
            for j in 0..m {
                if !(b[i][j] + 7 == b[i+1][j]) {
                    println!("No");
                    return;
                }
            }
            // if !(b[i][0] + 7 == b[i+1][0]) {
            //     println!("No");
            //     return;
            // }

        }
    }
    println!("Yes");
}
