// https://atcoder.jp/contests/abc218/tasks/abc218_c

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

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
    let mut t  = HashSet::new();
    let (mut tx, mut ty) = (0, 0);
    for i in 0..N {
        for j in 0..N {
            if T[i][j] == '#' {
                t.insert((i as isize, j as isize));
                (tx, ty) = (i as isize, j as isize);
            }
        }
    }
    if S.iter().flat_map(|x| x.iter()).filter(|&x| *x == '#').count() != t.len() {
        println!("No");
        return;
    }
    let M = t.len();
    for _ in 0..4 {
        let mut s = Vec::new();
        for i in 0..N {
            for j in 0..N {
                if S[i][j] == '#' {
                    s.push((i as isize, j as isize));
                }
            }
        }
        'pass: for i in 0..M {
            let x = tx - s[i].0;
            let y = ty - s[i].1;
            for j in 0..M {
                let nx = s[j].0 + x;
                let ny = s[j].1 + y;
                if !t.contains(&(nx, ny)) {
                    continue 'pass;
                }
            }
            println!("Yes");
            return;
        }
        let mut s = vec![vec!['?'; N]; N];
        for i in 0..N {
            for j in 0..N {
                s[N-j-1][i] = S[i][j];
            }
        }
        S = s;
    }
    println!("No");
}