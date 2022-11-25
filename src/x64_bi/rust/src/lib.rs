extern crate ascon;
use ascon::ascon_128_encrypt;
use ascon::ascon_128_decrypt;
use ascon::ascon_128_a_encrypt;
use ascon::ascon_128_a_decrypt;


fn align(x: &Vec<u8>) -> Vec<u8> {
    let mut _x = x.clone();
    _x.resize(x.len() + 8 - (x.len() % 8), 0);
    _x
}

pub fn ascon128_encrypt(m: Vec<u8>, a: Vec<u8>, n: [u8; 16], k: [u8; 16]) -> (Vec<u8>, u32) {
    let _m = align(&m);
    let _a = align(&a);
    let mut _c = vec![0; _m.len() + 16];

    let res = ascon_128_encrypt(
        _m.as_ptr(),
        _c.as_mut_ptr(),
        m.len().try_into().unwrap(),
        _a.as_ptr(),
        a.len().try_into().unwrap(),
        [n, k].concat().try_into().unwrap());

    ([&_c[16..(m.len()+16)], &_c[..16]].concat(), res)
}

pub fn ascon128_decrypt(c: Vec<u8>, a: Vec<u8>, n: [u8; 16], k: [u8; 16]) -> (Vec<u8>, u32) {
    if c.len() < 16 {
        panic!();
    }
    let _c_tag = [&c[(c.len()-16)..], &c[..(c.len()-16)]].concat();
    let mut _c = align(&_c_tag);
    let mut _a = align(&a);
    let mut _m = vec![0; _c.len() - 16];

    let res = ascon_128_decrypt(
        _m.as_mut_ptr(),
        _c.as_ptr(),
        c.len().try_into().unwrap(),
        _a.as_ptr(),
        a.len().try_into().unwrap(),
        [n, k].concat().try_into().unwrap());

    (_m[..(c.len()-16)].to_vec(), res)
}


pub fn ascon128a_encrypt(m: Vec<u8>, a: Vec<u8>, n: [u8; 16], k: [u8; 16]) -> (Vec<u8>, u32) {
    let _m = align(&m);
    let _a = align(&a);
    let mut _c = vec![0; _m.len() + 16];

    let res = ascon_128_a_encrypt(
        _m.as_ptr(),
        _c.as_mut_ptr(),
        m.len().try_into().unwrap(),
        _a.as_ptr(),
        a.len().try_into().unwrap(),
        [n, k].concat().try_into().unwrap());

    ([&_c[16..(m.len()+16)], &_c[..16]].concat(), res)
}

pub fn ascon128a_decrypt(c: Vec<u8>, a: Vec<u8>, n: [u8; 16], k: [u8; 16]) -> (Vec<u8>, u32) {
    if c.len() < 16 {
        panic!();
    }
    let _c_tag = [&c[(c.len()-16)..], &c[..(c.len()-16)]].concat();
    let mut _c = align(&_c_tag);
    let mut _a = align(&a);
    let mut _m = vec![0; _c.len() - 16];

    let res = ascon_128_a_decrypt(
        _m.as_mut_ptr(),
        _c.as_ptr(),
        c.len().try_into().unwrap(),
        _a.as_ptr(),
        a.len().try_into().unwrap(),
        [n, k].concat().try_into().unwrap());

    (_m[..(c.len()-16)].to_vec(), res)
}
