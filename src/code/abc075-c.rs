// https://atcoder.jp/contests/abc075/tasks/abc075_c

use proconio::input;
use proconio::fastout;
use ac_library::Dsu;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
    }
    let mut edge = Vec::new();
    for _ in 0..M {
        input! {
            (a, b): (usize, usize),
        }
        edge.push((a-1, b-1));
    }
    let mut ans = 0;
    for i in 0..M {
        let mut UF = Dsu::new(N);
        for j in 0..M {
            if i == j {
                continue;
            }
            UF.merge(edge[j].0, edge[j].1);
        }
        if UF.size(0) < N {
            ans += 1;
        }
    }
    println!("{}", ans);
}