use crate::util::*;

pub struct Rsa {
    n: i128,
    e: i128,
    d: i128,
}
impl Rsa {
    pub fn new() -> Rsa {
        let p = 7753;
        let q = 7907;
        let e = 65537;
        let lamda = (p - 1) * (q - 1) / gcd(p - 1, q - 1);
        let d = modular_multiplicative_inverse(e, lamda);
        Rsa { n: p * q, e, d }
    }
    pub fn get_pub_key(&self) -> (i128, i128) {
        (self.n, self.e)
    }
    pub fn encrypt(&self, (n, e): &(i128, i128), m: i128) -> i128 {
        modular_exponent(m, *e, *n)
    }
    pub fn decrypt(&self, c: i128) -> i128 {
        modular_exponent(c, self.d, self.n)
    }
}
