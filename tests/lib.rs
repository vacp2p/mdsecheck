use ark_bn254::Fr;
use mdsecheck::{mat, random_cauchy, security_level};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

#[test]
fn test_random_cauchy() {
    let mut r = ChaCha8Rng::seed_from_u64(123);
    let b = [Fr::from(1), Fr::from(2), Fr::from(3), Fr::from(4)];
    for (_, n) in (0..100).zip((1..=b.len()).cycle()) {
        let m = random_cauchy::<Fr>(n as u32, &mut r).unwrap();
        // Checking the generated matrix for nonsingularity
        mat::system_solution(&m, &b[..n]).unwrap();
    }
}

#[test]
fn test_security_level() {
    let mut r = ChaCha8Rng::seed_from_u64(456);
    for (i, n) in (0..12).zip((2..=7).cycle()) {
        let m = random_cauchy::<Fr>(n, &mut r).unwrap();
        let s = security_level(&m, 25);
        match i {
            // The expected output has been computed by means of a SageMath script, which uses
            // the built-in tools for working with matrices and polynomials over finite fields.
            // In particular, the methods for computing the characteristic polynomial of a matrix
            // and checking the irreducibility of a polynomial have been used
            0 | 7 | 9 => assert_eq!(s, Some(25)),
            _ => assert_eq!(s, None),
        }
    }
}
