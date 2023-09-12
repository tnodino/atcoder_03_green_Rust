// https://atcoder.jp/contests/abc185/tasks/abc185_f

use proconio::input;
use proconio::fastout;
use std::convert::Infallible;
use std::marker::PhantomData;
use std::ops::BitXor;
use ac_library::{Monoid, Segtree};

pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_integral {
    ($($ty:ty),*) => {
        $(
            impl Zero for $ty {
                #[inline]
                fn zero() -> Self {
                    0
                }
            }
        )*
    };
}

impl_integral!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

pub struct BitwiseXor<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for BitwiseXor<S>
where
    S: Copy + BitXor<Output = S> + Zero,
{
    type S = S;
    fn identity() -> Self::S {
        S::zero()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a ^ *b
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize, usize),
        A: [usize; N],
    }
    let mut Seg = Segtree::<BitwiseXor<usize>>::new(N+1);
    for i in 1..=N {
        Seg.set(i, A[i-1]);
    }
    for _ in 0..Q {
        input! {
            (T, X, Y): (usize, usize, usize),
        }
        match T {
            1 => {
                Seg.set(X, Seg.get(X) ^ Y);
            },
            2 => {
                println!("{}", Seg.prod(X..=Y));
            },
            _ => {},
        }
    }
}