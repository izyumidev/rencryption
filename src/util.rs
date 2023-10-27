use core::f64;
use std::{i128, i8};

pub fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn modular_multiplicative_inverse(a: u128, m: u128) -> u128 {
    for x in 1..m {
        if (a * x) % m == 1 {
            return x;
        }
    }
    0
}

pub fn modular_exponent(mut b: u128, mut e: u128, m: u128) -> u128 {
    let mut c = 1;
    b %= m;
    while e > 0 {
        if e % 2 == 1 {
            c = (c * b) % m;
        }
        e >>= 1;
        b = (b * b) % m;
    }
    c
}

pub fn gen_large_prime() -> u128 {
    let mut prime = random_number();
    loop {
        if check_prime(prime) {
            break;
        }
        prime = random_number()
    }
    prime
}

pub fn random_number() -> u128 {
    rand::random::<u128>()
}

// https://en.wikipedia.org/wiki/Baillie%E2%80%93PSW_primality_test
pub fn check_prime(n: u128) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    // base 2 strong probable prime test https://en.wikipedia.org/wiki/Strong_pseudoprime
    if modular_exponent(2, n, n) != 1 {
        return false;
    }

    // https://en.wikipedia.org/wiki/Jacobi_symbol
    // file the first D in the sequence 5, -7, 9, -11, ...
    // such that (D/n) = -1, where (D/n) signify the Jacobi symbol
    let mut d: i128 = 5;
    while modular_exponent((d + n as i128) as u128, (n - 1) / 2, n) != n - 1 {
        if d > 0 {
            d += 2
        } else {
            d -= 2
        }
        d *= -1;
    }
    let p = 1;
    let q = (1 - d) / 4;

    true
}
