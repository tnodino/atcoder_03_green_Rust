// https://atcoder.jp/contests/atc002/tasks/abc007_3

use proconio::input;
use proconio::fastout;
use std::usize::MAX;
use std::collections::VecDeque;

const DX: &[usize] = &[!0, 1, 0, 0];
const DY: &[usize] = &[0, 0, !0, 1];

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        R: usize,
        C: usize,
        sy: usize,
        sx: usize,
        gy: usize,
        gx: usize,
    }
    let sy = sy - 1;
    let sx = sx - 1;
    let gy = gy - 1;
    let gx = gx - 1;
    let mut c = Vec::new();
    for _ in 0..R {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        c.push(s);
    }
    let mut cnt = vec![vec![MAX; C]; R];
    cnt[sy][sx] = 0;
    let mut que = VecDeque::new();
    que.push_back((sy, sx));
    while !que.is_empty() {
        let (x, y) = que.pop_front().unwrap();
        for d in 0..4 {
            let nx = x.wrapping_add(DX[d]);
            let ny = y.wrapping_add(DY[d]);
            if R <= nx && C <= ny {
                continue;
            }
            if c[nx][ny] == '#' {
                continue;
            }
            if cnt[nx][ny] == MAX {
                cnt[nx][ny] = cnt[x][y] + 1;
                que.push_back((nx, ny));
            }
        }
    }
    println!("{}", cnt[gy][gx]);
}