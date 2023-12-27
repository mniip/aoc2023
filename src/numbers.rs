use std::{
    mem,
    ops::{Add, AddAssign, Div, Mul, Rem, Sub, SubAssign},
};

pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: PartialEq
        + Clone
        + From<bool>
        + Div
        + Mul<<T as Div>::Output>
        + SubAssign<<T as Mul<<T as Div>::Output>>::Output>,
{
    while b.ne(&T::from(false)) {
        let q = a.clone() / b.clone();
        a -= b.clone() * q;
        mem::swap(&mut a, &mut b);
    }
    a
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: PartialEq + Clone + Div + From<bool> + Mul<<T as Div>::Output, Output = T> + SubAssign<T>,
{
    let g = gcd(a.clone(), b.clone());
    b * (a / g)
}

#[derive(Clone, Copy, Debug)]
pub struct ExtendedGCD<T> {
    pub k_a: T,
    pub k_b: T,
    pub f_a: T,
    pub f_b: T,
    pub gcd: T,
    pub sign: bool,
}

/// If `e = bezout_identity(a, b)` then:
/// `e.gcd == gcd(a, b)``
/// `e.gcd == if e.sign { b * e.k_b - a * e.k_a } else { a * e.k_a - b * e.k_b }`
/// `e.f_a * e.gcd = a`
/// `e.f_b * e.gcd = b`
/// May panic only if `a`` or `b`` is `T::MIN``
pub fn bezout_identity<T>(mut a: T, mut b: T) -> ExtendedGCD<T>
where
    T: PartialEq
        + Clone
        + From<bool>
        + Div
        + Mul<<T as Div>::Output>
        + AddAssign<<T as Mul<<T as Div>::Output>>::Output>
        + SubAssign<T>
        + SubAssign<<T as Mul<<T as Div>::Output>>::Output>,
    <T as Div>::Output: Clone,
{
    let (mut k_a, mut k_b, mut f_b, mut f_a) =
        (T::from(true), T::from(false), T::from(false), T::from(true));
    let mut sign = false;
    while b.ne(&T::from(false)) {
        let q = a.clone() / b.clone();
        k_a += f_b.clone() * q.clone();
        mem::swap(&mut k_a, &mut f_b);
        k_b += f_a.clone() * q.clone();
        mem::swap(&mut k_b, &mut f_a);
        a -= b.clone() * q;
        mem::swap(&mut a, &mut b);
        sign = !sign;
    }
    ExtendedGCD {
        k_a,
        k_b,
        f_a,
        f_b,
        gcd: a,
        sign,
    }
}

pub fn chinese_remainder<T>(gcd: ExtendedGCD<T>, rem_a: T, rem_b: T) -> T
where
    T: PartialOrd
        + Clone
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Rem<T, Output = T>,
{
    let a = gcd.gcd.clone() * gcd.f_a.clone();
    let b = gcd.gcd * gcd.f_b.clone();
    let m = b.clone() * gcd.f_a.clone();
    if gcd.sign {
        (rem_a * gcd.k_b % a * gcd.f_b + (b.clone() - rem_b) * gcd.k_a % b * gcd.f_a) % m
    } else {
        (rem_b * gcd.k_a % b * gcd.f_a + (a.clone() - rem_a) * gcd.k_b % a * gcd.f_b) % m
    }
}
