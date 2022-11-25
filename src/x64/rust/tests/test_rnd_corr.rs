use ascon_jasmin::{ascon128a_decrypt, ascon128a_encrypt};
use rand::{distributions::Alphanumeric, Rng};

fn gen_random(len: usize) -> Vec<u8> {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len).map(char::from).collect();
    return s.as_bytes().to_vec();
}

fn assert_decrypt_jasmin(c: Vec<u8>, a: Vec<u8>, m_exp: Vec<u8>, k: [u8; 16], n: [u8; 16]) {
    let res: u64;
    let mut _m;

    (_m, res) = ascon128a_decrypt(c.clone(), a, k, n);

    assert_eq!(res, 0, "\n m = {:?} \n mlen = {:?} \n k = {:?} \n c = {:?}",     _m, _m.len(), k, c);
    assert_eq!(_m, m_exp,  "\n m = {:?} \n mlen = {:?} \n k = {:?} \n c = {:?}", _m, _m.len(), k, c);
}

fn assert_correctness_jasmin(m: Vec<u8>, a: Vec<u8>, k: [u8; 16], n: [u8; 16]) {
    let res: u64;
    let mut _c;

    (_c, res) = ascon128a_encrypt(m.clone(), a.clone(), k, n);

    assert_eq!(res, 0, "\n m = {:?} \n mlen = {:?} \n k = {:?} \n c = {:?}", m, m.len(), k, _c);
    assert_decrypt_jasmin(_c, a, m, k, n);
}

#[test]
fn correctness_plaintext_length_zero_nonce_zero_key() {
    for len in 0..1024 {
        let m = gen_random(len);
        let a = vec![0;0];
        let k = [0;16];
        let n = [0;16];
        
        assert_correctness_jasmin(m, a, k, n);
    }
}

#[test]
fn correctness_plaintext_length_random_nonce_random_key() {
    for len in 0..1024 {
        let m = gen_random(len);
        let a = vec![0;0];
        let k: [u8;16] = rand::random();
        let n: [u8;16] = rand::random();
        
        assert_correctness_jasmin(m, a, k, n);
    }
}

#[test]
fn correctness_plaintext_and_ad_length_zero_nonce_zero_key() {
    for len in 0..1024 {
        let m = gen_random(len);
        let a = gen_random(len);
        let k = [0;16];
        let n = [0;16];
        
        assert_correctness_jasmin(m, a, k, n);
    }
}

#[test]
fn correctness_plaintext_and_ad_length_random_nonce_random_key() {
    for len in 0..1024 {
        let m = gen_random(len);
        let a = gen_random(len);
        let k: [u8;16] = rand::random();
        let n: [u8;16] = rand::random();
        
        assert_correctness_jasmin(m, a, k, n);
    }
}

#[test]
fn correctness_tag_manipulation() {
    for len in 0..1024 {
        let mut res: u64;
        let m = gen_random(len);
        let a = gen_random(len);
        let k: [u8;16] = rand::random();
        let n: [u8;16] = rand::random();
        let mut _c;
        let mut _m = m.clone();
        _m.fill(0);

        (_c, res) = ascon128a_encrypt(m.clone(), a.clone(), k, n);

        assert_eq!(res, 0, "\n m = {:?} \n mlen = {:?} \n k = {:?} \n c = {:?}", m, m.len(), k, _c);

        let r: u8 = rand::random();
        _c.resize(_c.len() - 5 , r);
        _c.resize(_c.len() + 5 , r);
            
        (_m, res) = ascon128a_decrypt(_c.clone(), a, k, n);

        assert_eq!(res, u64::MAX, "\n m = {:?} \n mlen = {:?} \n k = {:?} \n c = {:?}", _m, _m.len(), k, _c);
        assert_eq!(m, _m,  "\n m = {:?} \n mlen = {:?} \n k = {:?} \n c = {:?}", _m, _m.len(), k, _c);
    }
}
