use proconio::input;
fn main() {
    input! {
        c: [[isize; 3]; 3],
    }

    for i in 0..c[0][0]+1 {
        let a1 = i;
        let b1 = c[0][0] - i;
        if b1 < 0 {continue};

        let mut a2 = 0;
        let mut b2 = 0;
        if (c[1][0] - b1) + (c[0][1] - a1) == c[1][1] {
            a2 = c[1][0] - b1;
            b2 = c[0][1] - a1;
        } else { continue }

        if (c[2][0] - b1) == (c[2][1] - b2) &&
            (c[0][2] - a1) == (c[1][2] - a2) &&
            (c[2][0] -b1) + (c[0][2] - a1) == c[2][2] {
            println!("Yes");
            return;
        }
    }
    println!("No");
    
}
