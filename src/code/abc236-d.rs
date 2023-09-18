// https://atcoder.jp/contests/abc236/tasks/abc236_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[allow(non_snake_case)]
fn dfs(vec: &mut Vec<(usize, usize)>, flg: &mut Vec<bool>, N: usize, M: usize, A: &Vec<Vec<usize>>) -> usize {
    let mut ret = 0;
    if vec.len() == N {
        for i in 0..N {
            ret ^= A[vec[i].0][vec[i].1];
        }
        return ret;
    }
    let mut l = 0;
    for i in 0..M {
        if !flg[i] {
            l = i;
            break;
        }
    }
    flg[l] = true;
    for r in l+1..M {
        if !flg[r] {
            vec.push((l, r));
            flg[r] = true;
            ret = max(ret, dfs(vec, flg, N, M, &A));
            vec.pop();
            flg[r] = false;
        }
    }
    flg[l] = false;
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = N * 2;
    let mut A = vec![vec![0; M]; M];
    for i in 0..M-1 {
        input! {
            a: [usize; M-i-1],
        }
        for j in i+1..M {
            A[i][j] = a[j-i-1];
        }
    }
    let mut vec = Vec::new();
    let mut flg = vec![false; M];
    println!("{}", dfs(&mut vec, &mut flg, N, M, &A));
}