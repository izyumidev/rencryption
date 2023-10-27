use std::time::Instant;

mod rsa;
mod util;
use rsa::Rsa;

fn main() {
    rsa();
}

fn rsa() {
    let start = Instant::now();
    println!("starting rsa");
    let alice = Rsa::new();
    let bob = Rsa::new();
    println!("keys created: {:?}", start.elapsed());
    let bob_pub_key = &bob.get_pub_key();
    let secret_mes = 65;
    println!("secret mes: {}", secret_mes);
    let encrypted_mes = alice.encrypt(bob_pub_key, secret_mes);
    println!("encrypted mes: {}", encrypted_mes);
    println!("decrypted mes: {}", bob.decrypt(encrypted_mes));
    println!("completed in: {:?}", start.elapsed());
    println!("ending rsa");
}
