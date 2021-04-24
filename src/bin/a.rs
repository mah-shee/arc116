#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::collections::HashMap;
#[fastout]
fn main() {
    input! {
        t: usize,
        case: [usize; t],
    }
    for i in 0..t {
        let primes = trial_division(case[i]);
        let mut total = 1;
        let mut odd_only = 1;
        for (key, value) in primes.iter() {
            if key % 2 == 0 {
                total *= value + 1;
            } else {
                odd_only *= value + 1;
                total *= value + 1;
            }
        }
        if total - odd_only == odd_only {
            println!("Same");
        } else if total - odd_only < odd_only {
            println!("Odd");
        } else {
            println!("Even");
        }
    }
}
fn trial_division(mut n: usize) -> HashMap<usize, usize> {
    let mut primes = HashMap::new();
    let mut i = 2;

    //  n を root n 以下の整数で割り切れるまで割っていく
    while i * i <= n {
        while n % i == 0 {
            n /= i;
            if primes.contains_key(&i) {
                let x = primes.get_mut(&i).unwrap();
                *x += 1;
            } else {
                primes.insert(i, 1);
            }
        }
        i += 1;
    }

    // 最後にnが素数になっている場合はそれ自身も素因数に含めて終了
    if n > 1 {
        if primes.contains_key(&n) {
            let x = primes.get_mut(&n).unwrap();
            *x += 1;
        } else {
            primes.insert(n, 1);
        }
    }
    primes
}
