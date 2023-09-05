// https://atcoder.jp/contests/abc193/tasks/abc193_d

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn check4(N: usize, S: String, cnt: &mut Vec<usize>) -> Vec<usize> {
    let mut cnt4 = vec![0; N];
    for s in S.chars() {
        if s == '#' {
            break;
        }
        let idx = (s as usize) - ('0' as usize);
        cnt[idx] -= 1;
        cnt4[idx] += 1;
    }
    return cnt4;
}

#[allow(non_snake_case)]
fn score(N: usize, vec: &Vec<usize>, pow10: &Vec<usize>) -> usize {
    let mut ret = 0;
    for i in 1..N {
        ret += i * pow10[vec[i]];
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let N = 10;
    input! {
        K: usize,
        S: String,
        T: String,
    }
    let mut pow10 = vec![1; N];
    for i in 1..N {
        pow10[i] = pow10[i-1] * 10;
    }
    let mut cnt = vec![K; N];
    let mut cnts = check4(N, S, &mut cnt);
    let mut cntt = check4(N, T, &mut cnt);
    let game = (K * 9 - 8) * (K * 9 - 9);
    let mut win = 0;
    for s in 1..N {
        for t in 1..N {
            if s == t && cnt[s] <= 1 {
                continue;
            }
            if s != t && cnt[s] == 0 && cnt[t] == 0 {
                continue;
            }
            cnts[s] += 1;
            cntt[t] += 1;
            if score(N, &cnts, &pow10) > score(N, &cntt, &pow10) {
                win += match s == t {
                    true => cnt[s] * (cnt[s] - 1),
                    false => cnt[s] * cnt[t],
                };
            }
            cnts[s] -= 1;
            cntt[t] -= 1;
        }
    }
    println!("{}", win as f64 / game as f64);
}