#[no_mangle]
pub fn ascon_128_encrypt(m: *const u8, c: *mut u8, mlen: u64, a: *const u8, alen: u64, p: [u8; 32]) -> u64 {
    return 0;
}

#[no_mangle]
pub fn ascon_128_decrypt(m: *mut u8, c: *const u8, clen: u64, a: *const u8, alen: u64, p: [u8; 32]) -> u64 {
    return 0;
}

#[no_mangle]
pub fn ascon_128_a_encrypt(m: *const u8, c: *mut u8, mlen: u64, a: *const u8, alen: u64, p: [u8; 32]) -> u64 {
    return 0;
}

#[no_mangle]
pub fn ascon_128_a_decrypt(m: *mut u8, c: *const u8, clen: u64, a: *const u8, alen: u64, p: [u8; 32]) -> u64 {
    return 0;
}
