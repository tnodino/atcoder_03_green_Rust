// https://atcoder.jp/contests/abc050/tasks/arc066_a

use proconio::input;
use proconio::fastout;
use ac_library::ModInt1000000007 as Mint;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut cnt = vec![0; N];
    for i in 0..N {
        cnt[A[i]] += 1;
    }
    for i in 0..N {
        let idx = ((N as isize - i as isize - 1) - (i as isize)).abs() as usize;
        if cnt[idx] == 0 {
            println!("0");
            return;
        }
        cnt[idx] -= 1;
    }
    println!("{}", Mint::new(2).pow((N/2).try_into().unwrap()));
}