use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        n: usize,
        mut s: Chars
    };


    s.insert(0, '0');

    let mut r = 0i64;
    let mut g = 0i64;
    let mut b = 0i64;

    for &si in &s{
        if si == 'R' {
            r += 1
        }
        else if si == 'G' {
            g += 1
        }
        else if si == 'B' {
            b += 1
        }
    }

    let mut ans = r * g * b;

    for j in 2..=n-1 {
        let mut i = j - 1;
        let mut k = j + 1;
        while 1 <= i && k <= n {
            if s[i] != s[j] && s[j] != s[k] && s[k] != s[i]{
                ans -= 1
            }
            i -= 1;
            k += 1;
        }
    }

    println!("{}", ans);
}
