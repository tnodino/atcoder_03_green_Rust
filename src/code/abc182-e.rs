// https://atcoder.jp/contests/abc182/tasks/abc182_e

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W, N, M): (usize, usize, usize, usize),
    }
    let mut grid = vec![vec!['-'; W]; H];
    for _ in 0..N {
        input! {
            (A, B): (usize, usize),
        }
        grid[A-1][B-1] = 'o';
    }
    for _ in 0..M {
        input! {
            (C, D): (usize, usize),
        }
        grid[C-1][D-1] = 'x';
    }
    let mut row = vec![vec![false; W]; H];
    let mut column = vec![vec![false; W]; H];
    for i in 0..H {
        for j in 0..W {
            if grid[i][j] == 'o' && !row[i][j] {
                let mut y = j;
                while y < W && grid[i][y] != 'x' {
                    row[i][y] = true;
                    y = y.wrapping_add(!0);
                }
                let mut y = j;
                while y < W && grid[i][y] != 'x' {
                    row[i][y] = true;
                    y = y.wrapping_add(1);
                }
            }
            if grid[i][j] == 'o' && !column[i][j] {
                let mut x = i;
                while x < H && grid[x][j] != 'x' {
                    column[x][j] = true;
                    x = x.wrapping_add(!0);
                }
                let mut x = i;
                while x < H && grid[x][j] != 'x' {
                    column[x][j] = true;
                    x = x.wrapping_add(1);
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..H {
        for j in 0..W {
            if row[i][j] || column[i][j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}