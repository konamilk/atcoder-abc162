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
        n: i64
    };

    let  mut ans = 0i64;

    for i in 1..=n{
        if i % 3 == 0{
            continue
        }
        if i % 5 == 0{
            continue
        }
        ans += i;
    }

    println!("{}", ans);
}
