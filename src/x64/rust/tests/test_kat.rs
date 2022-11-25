use ascon_jasmin::{ascon128a_encrypt, ascon128a_decrypt, ascon128_decrypt, ascon128_encrypt};
// Adapted from https://github.com/sebastinas/ascon-aead/blob/main/ascon-aead/tests/kats_test.rs
// Copyright 2022 Sebastian Ramacher
// SPDX-License-Identifier: MIT
//
use spectral::prelude::{asserting, OrderedAssertions};
use std::collections::HashMap;
use std::include_str;


#[derive(Debug)]
struct TestVector {
    count: u32,
    key: [u8; 16],
    nonce: [u8; 16],
    plaintext: Vec<u8>,
    associated_data: Vec<u8>,
    ciphertext: Vec<u8>,
}

impl TestVector {
    fn new(
        count: &str,
        key: &str,
        nonce: &str,
        plaintext: &str,
        associated_data: &str,
        ciphertext: &str,
    ) -> Self {

        let mut _n = hex::decode(nonce).unwrap();
        _n.resize(16, 0);
        let n: [u8; 16] = _n.try_into().unwrap();

        let mut _k = hex::decode(key).unwrap();
        _k.resize(16, 0);
        let k: [u8; 16] = _k.try_into().unwrap();

        Self {
            count: count.parse().unwrap(),
            key: k,
            nonce: n,
            plaintext: hex::decode(plaintext).unwrap(),
            associated_data: hex::decode(associated_data).unwrap(),
            ciphertext: hex::decode(ciphertext).unwrap(),
        }
    }
}

fn run_tv_ascon_128(tv: TestVector) {
    let mut res: u64;
    let c: Vec<u8>;
    let m: Vec<u8>;
    (c, res) = ascon128_encrypt(tv.plaintext.clone(), tv.associated_data.clone(), tv.nonce, tv.key);
    asserting(format!("Test Vector {} encryption", tv.count).as_str())
        .that(&c)
        .is_equal_to(&tv.ciphertext);
    asserting(format!("Test Vector {} encryption", tv.count).as_str())
        .that(&res)
        .is_equal_to(0);

    (m, res) = ascon128_decrypt(tv.ciphertext, tv.associated_data, tv.nonce, tv.key);
    asserting(format!("Test Vector {} decryption", tv.count).as_str())
        .that(&m)
        .is_equal_to(&tv.plaintext);
    asserting(format!("Test Vector {} decryption result", tv.count).as_str())
        .that(&res)
        .is_equal_to(0);
}

fn run_tv_ascon_128_a(tv: TestVector) {
    let mut res: u64;
    let c: Vec<u8>;
    let m: Vec<u8>;
    (c, res) = ascon128a_encrypt(tv.plaintext.clone(), tv.associated_data.clone(), tv.nonce, tv.key);
    asserting(format!("Test Vector {} encryption", tv.count).as_str())
        .that(&c)
        .is_equal_to(&tv.ciphertext);
    asserting(format!("Test Vector {} encryption", tv.count).as_str())
        .that(&res)
        .is_equal_to(0);

    (m, res) = ascon128a_decrypt(tv.ciphertext, tv.associated_data, tv.nonce, tv.key);
    asserting(format!("Test Vector {} decryption", tv.count).as_str())
        .that(&m)
        .is_equal_to(&tv.plaintext);
    asserting(format!("Test Vector {} decryption result", tv.count).as_str())
        .that(&res)
        .is_equal_to(0);
}

fn parse_tvs(tvs: &str) -> Vec<TestVector> {
    let mut fields: HashMap<String, String> = HashMap::new();
    let mut ret = Vec::new();

    for line in tvs.lines() {
        if line.is_empty() && !fields.is_empty() {
            ret.push(TestVector::new(
                &fields["Count"],
                &fields["Key"],
                &fields["Nonce"],
                &fields["PT"],
                &fields["AD"],
                &fields["CT"],
            ));
            fields.clear();
            continue;
        }

        let mut values = line.split(" = ");
        fields.insert(
            values.next().unwrap().to_string(),
            values.next().unwrap().to_string(),
        );
    }

    asserting!("Test Vectors available")
        .that(&ret.len())
        .is_greater_than(0);
    ret
}

#[test]
fn test_vectors_ascon128() {
    let tvs = parse_tvs(include_str!("data/ascon_128_kat.txt"));
    for tv in tvs {
        run_tv_ascon_128(tv);
    }
}

#[test]
fn test_vectors_ascon128a() {
    let tvs = parse_tvs(include_str!("data/ascon_128_a_kat.txt"));
    for tv in tvs {
        run_tv_ascon_128_a(tv);
    }
}

