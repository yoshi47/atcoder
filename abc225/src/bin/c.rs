use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[usize; m]; n],
    };

    for i in 0..n {
        if b[i][0..m-1].iter().any(|&x| x % 7 == 0) {
            println!("No");
            return;
        }

        let tmp: Vec<usize> = (b[i][0]..b[i][0]+m).collect();
        if !(b[i] == tmp) {
            println!("No");
            return;
        }
        
        if !(i == n-1) {
            let tmp: Vec<usize> = b[i].iter().map(|&x| x + 7).collect();
            if !(b[i+1] == tmp) {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}