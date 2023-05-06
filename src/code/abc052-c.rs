// https://atcoder.jp/contests/abc052/tasks/arc067_a

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 2..=N {
        let mut x = i;
        for j in 2..=i {
            while x % j == 0 {
                x /= j;
                *map.entry(j).or_insert(0) += 1;
            }
        }
    }
    let mut ans = 1;
    for v in map.values() {
        ans *= v + 1;
        ans %= MOD;
    }
    println!("{}", ans);
}