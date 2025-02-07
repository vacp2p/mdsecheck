use mdsecheck::num::{prime_divisors, reversed_bits};

#[test]
fn test_prime_divisors() {
    assert_eq!(prime_divisors(0), vec![]);
    assert_eq!(prime_divisors(1), vec![]);
    assert_eq!(prime_divisors(2), vec![2]);
    assert_eq!(prime_divisors(3), vec![3]);
    assert_eq!(prime_divisors(5), vec![5]);
    assert_eq!(prime_divisors(512), vec![2]);
    assert_eq!(prime_divisors(729), vec![3]);
    assert_eq!(prime_divisors(625), vec![5]);
    assert_eq!(prime_divisors(121), vec![11]);
    assert_eq!(prime_divisors(257), vec![257]);
    assert_eq!(prime_divisors(323), vec![17, 19]);
    assert_eq!(prime_divisors(7500), vec![2, 3, 5]);
    assert_eq!(prime_divisors(26620), vec![2, 5, 11]);
    assert_eq!(prime_divisors(5898330), vec![2, 3, 5, 65537]);
}

#[test]
fn test_reversed_bits() {
    assert_eq!(reversed_bits(&[]).next(), None);
    assert_eq!(reversed_bits(&[0]).next(), None);
    assert_eq!(reversed_bits(&[0, 0]).next(), None);
    assert_eq!(
        reversed_bits(&[0xA, u64::MAX]).collect::<Vec<_>>(),
        [&[true; 64][..], &[false; 60], &[true, false, true, false]].concat()
    );
    assert_eq!(
        reversed_bits(&[0xB, 1 << 63, 0]).collect::<Vec<_>>(),
        [
            &[true][..],
            &[false; 63],
            &[false; 60],
            &[true, false, true, true]
        ]
        .concat()
    );
    assert_eq!(
        reversed_bits(&[0xC, 0xD]).collect::<Vec<_>>(),
        [
            &[true, true, false, true][..],
            &[false; 60],
            &[true, true, false, false]
        ]
        .concat()
    );
    assert_eq!(
        reversed_bits(&[0xE, 0xF, 0, 0]).collect::<Vec<_>>(),
        [&[true; 4][..], &[false; 60], &[true; 3], &[false]].concat()
    );
}
