use std::time::Instant;

fn main() {
    let start = Instant::now();
    let alice = RSA::new();
    let bob = RSA::new();
    println!("keys created: {:?}", start.elapsed());

    let bob_pub_key = &bob.get_pub_key();

    let secret_mes = 65;
    println!("secret mes: {}", secret_mes);
    let encrypted_mes = alice.encrypt(bob_pub_key, secret_mes);
    println!("encrypted mes: {}", encrypted_mes);

    println!("decrypted mes: {}", bob.decrypt(encrypted_mes));

    println!("completed in: {:?}", start.elapsed());
}

struct RSA {
    n: i128,
    e: i128,
    d: i128,
}
impl RSA {
    fn new() -> RSA {
        let _p = 7753;
        let _q = 7907;
        let _e = 65537;
        let _lamda = (_p - 1) * (_q - 1) / gcd(_p - 1, _q - 1);
        let _d = modular_multiplicative_inverse(_e, _lamda);
        return RSA {
            n: _p * _q,
            e: _e,
            d: _d,
        };
    }
    fn get_pub_key(&self) -> (i128, i128) {
        (self.n, self.e)
    }
    fn encrypt(&self, (n, e): &(i128, i128), m: i128) -> i128 {
        modular_exponent(m, *e, *n)
    }
    fn decrypt(&self, c: i128) -> i128 {
        modular_exponent(c, self.d, self.n)
    }
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn modular_multiplicative_inverse(a: i128, m: i128) -> i128 {
    for x in 1..m {
        if (a * x) % m == 1 {
            return x;
        }
    }
    0
}

fn modular_exponent(mut b: i128, mut e: i128, m: i128) -> i128 {
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
