// https://atcoder.jp/contests/abc146/tasks/abc146_d

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn dfs(pos: usize, pre: usize, ng: usize, N: usize, G: &Vec<Vec<(usize, usize)>>, ans: &mut Vec<usize>) {
    let mut cnt = 1;
    for (nxt, idx) in G[pos].iter() {
        if *nxt == pre {
            continue;
        }
        if ng == cnt {
            cnt += 1;
        }
        ans[*idx] = cnt;
        dfs(*nxt, pos, cnt, N, &G, ans);
        cnt += 1;
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut G = vec![Vec::new(); N];
    for i in 0..N-1 {
        input! {
            a: usize,
            b: usize,
        }
        G[a-1].push((b-1, i));
        G[b-1].push((a-1, i));
    }
    let mut ans = vec![1; N-1];
    dfs(0, N, 0, N, &G, &mut ans);
    println!("{}", ans.iter().max().unwrap());
    for i in 0..N-1 {
        println!("{}", ans[i]);
    }
}