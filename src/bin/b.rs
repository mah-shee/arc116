#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n:usize,
        mut a: [usize; n],
    }
    a.sort();
    let waru = 998244352;
    let mut ans = 0;
    for bit in 0..(1 << n - 1) {
        let mut vector = Vec::new();
        for i in 0..n {
            if (bit & (1 << i)) == 1 {
                vector.push(a[i]);
                println!("{}", a[i]);
            }
        }
        for k in vector.iter() {
            println!("{}: {}", bit, k);
        }
        ans += vector[0] * vector[vector.len() - 1];
        ans %= waru;
        println!("{}", ans);
    }
    println!("{}", ans % waru);
}
