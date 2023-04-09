// https://atcoder.jp/contests/abc054/tasks/abc054_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut A = Vec::new();
    for _ in 0..N {
        input! {
            a: String,
        }
        let a = a.chars().collect::<Vec<char>>();
        A.push(a);
    }
    let mut B = Vec::new();
    for _ in 0..M {
        input! {
            b: String,
        }
        let b = b.chars().collect::<Vec<char>>();
        B.push(b);
    }
    for i in 0..=N-M {
        for j in 0..=N-M {
            let mut flg = true;
            for x in 0..M {
                for y in 0..M {
                    if A[i+x][j+y] != B[x][y] {
                        flg = false;
                    }
                }
            }
            if flg {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}