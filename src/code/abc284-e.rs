// https://atcoder.jp/contests/abc284/tasks/abc284_e

use proconio::input;
use proconio::fastout;

const MAX: usize = 1_000_000;

#[allow(non_snake_case)]
fn dfs(pos: usize, G: &Vec<Vec<usize>>, flg: &mut Vec<bool>, ans: &mut usize) {
    *ans += 1;
    for nxt in G[pos].iter() {
        if *ans == MAX || flg[*nxt] {
            continue;
        }
        flg[*nxt] = true;
        dfs(*nxt, &G, flg, ans);
        flg[*nxt] = false;
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
    }
    let mut G = vec![Vec::new(); N];
    for _ in 0..M {
        input! {
            (u, v): (usize, usize),
        }
        G[u-1].push(v-1);
        G[v-1].push(u-1);
    }
    let mut flg = vec![false; N];
    flg[0] = true;
    let mut ans = 0;
    dfs(0, &G, &mut flg, &mut ans);
    println!("{}", ans);
}