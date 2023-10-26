pub fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn modular_multiplicative_inverse(a: i128, m: i128) -> i128 {
    for x in 1..m {
        if (a * x) % m == 1 {
            return x;
        }
    }
    0
}

pub fn modular_exponent(mut b: i128, mut e: i128, m: i128) -> i128 {
    let mut c = 1;
    b = b % m;
    while e > 0 {
        if e % 2 == 1 {
            c = (c * b) % m;
        }
        e >>= 1;
        b = (b * b) % m;
    }
    c
}
