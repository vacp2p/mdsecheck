use ark_bn254::Fr;
use ark_poly::{polynomial::univariate::DensePolynomial, DenseUVPolynomial};
use mdsecheck::poly::{coprimality, new, power_modulo, reduced_modulo};

#[test]
fn test_coprimality() {
    assert_eq!(coprimality(&new::<Fr>(&[0]), &new(&[0])), false);
    assert_eq!(coprimality(&new::<Fr>(&[2]), &new(&[0])), true);
    assert_eq!(coprimality(&new::<Fr>(&[0]), &new(&[4])), true);
    assert_eq!(coprimality(&new::<Fr>(&[2, 3]), &new(&[0])), false);
    assert_eq!(coprimality(&new::<Fr>(&[0]), &new(&[4, 5])), false);
    assert_eq!(coprimality(&new::<Fr>(&[3]), &new(&[3, 3])), true);
    assert_eq!(coprimality(&new::<Fr>(&[3, 3]), &new(&[3])), true);
    assert_eq!(coprimality(&new::<Fr>(&[4, 4]), &new(&[4, 4])), false);
    assert_eq!(coprimality(&new::<Fr>(&[1, 6, 9]), &new(&[1, 4, 4])), true);
    assert_eq!(coprimality(&new::<Fr>(&[1, 5, 6]), &new(&[1, 6, 9])), false);
}

#[test]
fn test_new() {
    assert_eq!(
        new(&[] as &[u64]),
        DensePolynomial::from_coefficients_slice(&[Fr::from(0)])
    );
    assert_eq!(
        new(&[0, 0, 0]),
        DensePolynomial::from_coefficients_slice(&[Fr::from(0)])
    );
    assert_eq!(
        new(&[1, 2, 3]),
        DensePolynomial::from_coefficients_slice(&[Fr::from(3), Fr::from(2), Fr::from(1)])
    );
    assert_eq!(
        new(&[0, -1, 2, -3]),
        DensePolynomial::from_coefficients_slice(&[-Fr::from(3), Fr::from(2), -Fr::from(1)])
    );
}

#[test]
fn test_power_modulo() {
    assert_eq!(power_modulo(&new::<Fr>(&[0]), &[0], &new(&[0])), None);
    assert_eq!(power_modulo(&new::<Fr>(&[0]), &[10], &new(&[0])), None);
    assert_eq!(power_modulo(&new::<Fr>(&[1, 0]), &[0], &new(&[0])), None);
    assert_eq!(power_modulo(&new::<Fr>(&[1, 0]), &[10], &new(&[0])), None);
    assert_eq!(
        power_modulo(&new::<Fr>(&[0]), &[0], &new(&[25])),
        Some(new(&[0]))
    );
    assert_eq!(
        power_modulo(&new::<Fr>(&[0]), &[10], &new(&[25])),
        Some(new(&[0]))
    );
    assert_eq!(
        power_modulo(&new::<Fr>(&[1, 0]), &[0], &new(&[25])),
        Some(new(&[0]))
    );
    assert_eq!(
        power_modulo(&new::<Fr>(&[1, 10]), &[10], &new(&[25])),
        Some(new(&[0]))
    );
    assert_eq!(
        power_modulo(&new::<Fr>(&[0]), &[0], &new(&[1, -1])),
        Some(new(&[1]))
    );
    assert_eq!(
        power_modulo(&new::<Fr>(&[0]), &[10], &new(&[1, 0, -1])),
        Some(new(&[0]))
    );
    assert_eq!(
        power_modulo(&new::<Fr>(&[1, 0]), &[0], &new(&[1, 0, 0, -3])),
        Some(new(&[1]))
    );
    assert_eq!(
        power_modulo(&new::<Fr>(&[1, 0]), &[101], &new(&[1, 0, 0, 0, -2])),
        Some(new(&[1 << 25, 0]))
    );
    assert_eq!(
        power_modulo(&new::<Fr>(&[1, 0]), &[25, 3], &new(&[1, 0, 0, 0, 0, -1])),
        Some(new(&[1, 0, 0, 0]))
    );
}

#[test]
fn test_reduced_modulo() {
    assert_eq!(
        reduced_modulo(&new::<Fr>(&[4, 5, 6, 7, 8]), &new(&[0])),
        None
    );
    assert_eq!(
        reduced_modulo(&new::<Fr>(&[9, 10, 11, 12, 13]), &new(&[14])),
        Some(new(&[0]))
    );
    assert_eq!(
        reduced_modulo(&new::<Fr>(&[15, 16]), &new(&[17, 18, 19, 20])),
        Some(new(&[15, 16]))
    );
    assert_eq!(
        reduced_modulo(&new::<Fr>(&[3, 18, 39, 30, 21]), &new(&[1, 6, 12, 8])),
        Some(new(&[3, 6, 21]))
    );
}
