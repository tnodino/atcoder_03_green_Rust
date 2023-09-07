// https://atcoder.jp/contests/abc218/tasks/abc218_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut S = Vec::new();
    for _ in 0..N {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    let mut T = Vec::new();
    for _ in 0..N {
        input! {
            t: String,
        }
        let t = t.chars().collect::<Vec<char>>();
        T.push(t);
    }
    for _ in 0..4 {
        for x in 0..N {
            'pass: for y in 0..N {
                for i in 0..N {
                    for j in 0..N {
                        if S[i][j] == '#' && (N <= i + x || N <= j + y) {
                            continue 'pass;
                        }
                        if S[i][j] != T[(i + x) % N][(j + y) % N] {
                            continue 'pass;
                        }
                    }
                }
                println!("Yes");
                return;
            }
        }
        for x in 0..N {
            'pass: for y in 0..N {
                for i in 0..N {
                    for j in 0..N {
                        if S[i][j] == '#' && (i < x || N <= j + y) {
                            continue 'pass;
                        }
                        if S[i][j] != T[(i + N - x) % N][(j + y) % N] {
                            continue 'pass;
                        }
                    }
                }
                println!("Yes");
                return;
            }
        }
        for x in 0..N {
            'pass: for y in 0..N {
                for i in 0..N {
                    for j in 0..N {
                        if S[i][j] == '#' && (N <= i + x || j < y) {
                            continue 'pass;
                        }
                        if S[i][j] != T[(i + x) % N][(j + N - y) % N] {
                            continue 'pass;
                        }
                    }
                }
                println!("Yes");
                return;
            }
        }
        for x in 0..N {
            'pass: for y in 0..N {
                for i in 0..N {
                    for j in 0..N {
                        if S[i][j] == '#' && (i < x || j < y) {
                            continue 'pass;
                        }
                        if S[i][j] != T[(i + N - x) % N][(j + N - y) % N] {
                            continue 'pass;
                        }
                    }
                }
                println!("Yes");
                return;
            }
        }
        let mut vec = vec![vec!['?'; N]; N];
        for i in 0..N {
            for j in 0..N {
                vec[N-j-1][i] = S[i][j];
            }
        }
        S = vec;
    }
    println!("No");
}