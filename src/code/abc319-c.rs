// https://atcoder.jp/contests/abc319/tasks/abc319_c

use proconio::input;
use proconio::fastout;
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let N = 3;
    let M = 9;
    let mut c = Vec::new();
    for _ in 0..3 {
        input! {
            i1: [usize; N],
        }
        c.push(i1);
    }
    let pair = [[(0, 0), (0, 1), (0, 2)],
        [(1, 0), (1, 1), (1, 2)],
        [(2, 0), (2, 1), (2, 2)],
        [(0, 0), (1, 0), (2, 0)],
        [(0, 1), (1, 1), (2, 1)],
        [(0, 2), (1, 2), (2, 2)],
        [(0, 0), (1, 1), (2, 2)],
        [(0, 2), (1, 1), (2, 0)],];
    let mut num = 0.;
    let mut den = 0.;
    'outer: for perm in (0..M).permutations(M) {
        den += 1.;
        let mut flg = vec![vec![0; N]; N];
        for i in 0..M {
            let x = perm[i] / N;
            let y = perm[i] % N;
            flg[x][y] = i;
        }
        for i in 0..8 {
            let mut vec = Vec::new();
            for j in 0..3 {
                vec.push((flg[pair[i][j].0][pair[i][j].1], c[pair[i][j].0][pair[i][j].1]))
            }
            vec.sort();
            if vec[0].1 == vec[1].1 && vec[0].1 != vec[2].1 {
                continue 'outer;
            }
        }
        num += 1.;
    }
    println!("{}", num / den);
}