# Rust Interface to Jasmin Implementation of Ascon-128

<!--toc:start-->
- [Requirements](#requirements)
- [Implementation](#implementation)
- [Benchmark](#benchmark)
- [Tests](#tests)
  - [KAT](#kat)
  - [Correctness](#correctness)
<!--toc:end-->

## Requirements
TODO

## Implementation
Currently implemented configurations:

- Ascon-128  (Encrypt + Decrypt)
- Ascon-128a (Encrypt + Decrypt)

## Benchmark
Implementation based on the benchmarks of the rust crate [`ascon-aead`](https://github.com/sebastinas/ascon-aead).

## Tests

### KAT
Tests against known values implemented in `tests/test_kat.rs`.
Implementation based on the tests of the rust crate [`ascon-aead`](https://github.com/sebastinas/ascon-aead).

KAT Sources:
- `ascon_128_kat.txt`
    - [GitHub](https://github.com/ascon/ascon-c/blob/main/crypto_aead/ascon128v12/LWC_AEAD_KAT_128_128.txt)
    - Commit Ref: 22feeca5d90de629fcea420b5555d8de7dfea7fb
- `ascon_128_a_kat.txt`
    - [GitHub](https://github.com/ascon/ascon-c/blob/main/crypto_aead/ascon128av12/LWC_AEAD_KAT_128_128.txt)
    - Commit Ref: 22feeca5d90de629fcea420b5555d8de7dfea7fb

### Correctness
Tests evaluating the correctness of the implementations implemented in `tests/test_rnd_corr.rs`.
Validates the relation of encryption <-> decryption of the provided implementations using random values. 
