use ascon_aead::{Ascon128a, Key, Nonce};
use ascon_aead::aead::{Aead, KeyInit, Payload};
use ascon_jasmin::{ascon128a_encrypt, ascon128a_decrypt};


fn print_hex_words(x: &Vec<u8>) {
    (0..x.len()).for_each(|i| {
        if i % 8 == 0 {
            print!(" ");
        }
        print!("{:02x}", x[i]);
    });
}


fn test_jasmin(m: Vec<u8>, a: Vec<u8>, n: [u8; 16], k: [u8; 16]) {
    let mut res: u64;
    let mut _c;
    let mut _m;

    println!("--- Initial:");
    print_hex_words(&m);
    println!();
    
    (_c, res) = ascon128a_encrypt(m, a.clone(), n, k);

    println!();
    println!("--- Encrypt:");
    print_hex_words(&_c);
    println!();
    println!("Result: {}", res); 

    (_m, res) = ascon128a_decrypt(_c, a, n, k);

    println!();
    println!("--- Decrypt:");
    print_hex_words(&_m);
    println!();
    println!("Result: {}", res); 
}

fn test_rust(m: Vec<u8>, a: Vec<u8>, n: [u8; 16], k: [u8; 16]) {
    let key = Key::<Ascon128a>::from_slice(&k);
    let cipher = Ascon128a::new(key);
    let nonce = Nonce::<Ascon128a>::from_slice(&n);

    println!("--- Initial:");
    print_hex_words(&m);
    println!();

    let payload = Payload { msg: m.as_ref(), aad: a.as_ref() };
    let ciphertext = cipher.encrypt(nonce, payload)
        .expect("encryption failure!");

    println!();
    println!("--- Encrypt:");
    print_hex_words(&ciphertext);
    println!();
}

fn demo() {
    let m = vec![
        50, 117, 99, 71, 55, 49, 86, 111, 
        114, 112, 68, 120, 68, 121, 87, 111, 
        121, 68, 51, 68, 121, 106, 84, 100, 
        83, 87, 89, 112, 88, 75, 53, 78, 
        111, 105, 56, 105, 116, 79 , 56, 117, 
        71, 78, 68, 112, 75, 108, 78, 87, 
        118, 88, 84, 71, 52, 105, 77, 78,
        80, 90, 8];
    let a = vec![00, 0x01, 0x02, 0x03];
    let k = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
    let n = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];

    println!("----------------");
    println!("> Rust Reference");
    println!("----------------");
    test_rust(m.clone(), a.clone(), n, k);

    println!();
    println!("----------------");
    println!("> Jasmin");
    println!("----------------");
    test_jasmin(m, a, n, k);
}

fn main() {
    demo();
}

