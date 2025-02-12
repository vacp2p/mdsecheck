# MDSECheck
The crate provides tools for generating random square [MDS matrices](https://en.wikipedia.org/wiki/MDS_matrix) over prime finite fields and applying the MDSECheck method to check such matrices for unconditional security as the components of affine permutation boxes of [partial substitution-permutation networks (P-SPNs)](https://eprint.iacr.org/2020/500.pdf), which are widespread designs of the modern symmetric ciphers and hash functions. The used data types of field elements and polynomials are provided by the crates [ark-ff](https://docs.rs/ark-ff) and [ark-poly](https://docs.rs/ark-poly). The auxiliary tools in the crate modules are accessible as well.

## Definition of unconditional P-SPN security of square MDS matrices
The round unconditional P-SPN security level of the square MDS matrix `M` is defined as `l` rounds, where `l` is a positive integer, if and only if `M` simultaneously satisfies the following conditions:
1. The [minimal polynomials](https://en.wikipedia.org/wiki/Minimal_polynomial_(linear_algebra)) of `M`, `M²`, ..., `Mˡ` have maximum degree and are irreducible.
2. The minimal polynomial of `Mˡ⁺¹` is not of maximum degree or not irreducible.

Theorem 8 in the paper ["Proving Resistance Against Infinitely Long Subspace Trails: How to Choose the Linear Layer"](https://eprint.iacr.org/2020/500.pdf) by L. Grassi, C. Rechberger and M. Schofnegger ensures that if the round unconditional P-SPN security level of a square MDS matrix is `l`, then "there is no infinitely long [subspace trail](https://eprint.iacr.org/2020/500.pdf) with/without active S-boxes of period less than or equal to `l`" regardless of the structure of the P-SPN affine permutation box using this matrix, but does not provide the same guarantees for larger periods. This independence from the P-SPN affine permutation boxes is the reason for using the term "unconditional security". Once an MDS matrix with the round unconditional P-SPN security level `l` has been chosen, it can protect any P-SPN with at most `l` rounds from the ["attacks based on infinitely long truncated differentials with probability 1"](https://eprint.iacr.org/2020/500.pdf).

## Implemented approach to the security checks
To check whether the round unconditional P-SPN security level of the specified matrix is no less than the specified bound, the crate provides the implementation of the MDSECheck method, whose name is derived from the words "MDS", "security", "elaborated" and "check". An incomplete description of this novel method can be found in [this report](https://notes.status.im/CVMoa6EcTmS2D4VPBCsH2w), while a reference to a more detailed article dedicated to MDSECheck is planned to be included in the next version of the crate.

## Usage example
```rust
use ark_bn254::Fr;
use mdsecheck::{random_cauchy, security_level};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

// Generating pseudorandom 5 x 5 MDS matrices over the BN254 scalar field until
// a matrix with the round unconditional P-SPN security level 25 is obtained
let mut r = ChaCha8Rng::seed_from_u64(123456);
loop {
    // The field is large enough to generate 5 x 5 Cauchy matrices
    let m = random_cauchy::<Fr>(5, &mut r).unwrap();
    if security_level(&m, 25) == Some(25) {
        println!("{:?}", m);
        break;
    }
}
```

## Disclaimer
The current version of this crate has not undergone a third-party security audit and is not intended for production use without proper security review.