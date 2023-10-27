use crate::util::*;

pub struct Rsa {
    n: u128,
    e: u128,
    d: u128,
}
impl Rsa {
    pub fn new() -> Rsa {
        let p = gen_large_prime();
        let q = gen_large_prime();
        let e = 65537;
        let lamda = (p - 1) * (q - 1) / gcd(p - 1, q - 1);
        let d = modular_multiplicative_inverse(e, lamda);
        Rsa { n: p * q, e, d }
    }
    pub fn get_pub_key(&self) -> (u128, u128) {
        (self.n, self.e)
    }
    pub fn encrypt(&self, (n, e): &(u128, u128), m: u128) -> u128 {
        modular_exponent(m, *e, *n)
    }
    pub fn decrypt(&self, c: u128) -> u128 {
        modular_exponent(c, self.d, self.n)
    }
}
