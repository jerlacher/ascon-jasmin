#[no_mangle]
pub fn ascon_128_encrypt(m: *const u8, c: *mut u8, mlen: u32, a: *const u8, alen: u32, p: [u8; 32]) -> u32 {
    return 0;
}

#[no_mangle]
pub fn ascon_128_decrypt(m: *mut u8, c: *const u8, clen: u32, a: *const u8, alen: u32, p: [u8; 32]) -> u32 {
    return 0;
}

#[no_mangle]
pub fn ascon_128_a_encrypt(m: *const u8, c: *mut u8, mlen: u32, a: *const u8, alen: u32, p: [u8; 32]) -> u32 {
    return 0;
}

#[no_mangle]
pub fn ascon_128_a_decrypt(m: *mut u8, c: *const u8, clen: u32, a: *const u8, alen: u32, p: [u8; 32]) -> u32 {
    return 0;
}
