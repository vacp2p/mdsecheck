use ark_bn254::Fr;
use mdsecheck::{mat, random_cauchy, secure_rounds};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

#[test]
fn test_random_cauchy() {
    let mut r = ChaCha8Rng::seed_from_u64(123);
    let b = [Fr::from(1), Fr::from(2), Fr::from(3), Fr::from(4)];
    for (_, n) in (0..100).zip((1..=b.len()).cycle()) {
        let m = random_cauchy::<Fr>(n as u32, &mut r).unwrap();
        mat::unique_solution(&m, &b[..n]).unwrap();
    }
}

#[test]
fn test_secure_rounds() {
    let mut r = ChaCha8Rng::seed_from_u64(456);
    for (i, n) in (0..12).zip((2..=7).cycle()) {
        let m = random_cauchy::<Fr>(n, &mut r).unwrap();
        let s = secure_rounds(&m, 25);
        match i {
            0 | 7 | 9 => assert_eq!(s, Some(25)),
            _ => assert_eq!(s, None),
        }
    }
}
