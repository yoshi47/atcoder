use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };
    let pt: Vec<_> = (1..n+1).collect();
    let pp = p.to_vec();
    if pt == pp {
        println!("YES");
        return;
    }
    
    for i in 0..n-1 {
        for j in i+1..n {
            let mut pp = p.to_vec();
            pp.swap(i, j);
            if pt == pp {
                println!("YES");
                return;
            }
        }
    }
    println!("NO");
}
