//! Provides auxiliary tools for working with polynomials.

use ark_ff::{PrimeField, Zero};
use ark_poly::{
    polynomial::univariate::{DenseOrSparsePolynomial, DensePolynomial},
    DenseUVPolynomial, Polynomial,
};

/// Checks whether the arguments are coprime polynomials by means of the Euclidean method.
pub fn coprimality<F: PrimeField>(a: &DensePolynomial<F>, b: &DensePolynomial<F>) -> bool {
    let mut a = match reduced_modulo(a, b) {
        Some(r) => r,
        None => return !a.is_zero() && (a.degree() == 0),
    };
    let mut b = match reduced_modulo(b, &a) {
        Some(r) => r,
        None => return b.degree() == 0,
    };
    while let Some(r) = reduced_modulo(&a, &b) {
        (a, b) = (b, r);
    }
    a.degree() == 0
}

/// Creates a polynomial from the specified big-endian slice of coefficients.
pub fn new<F: PrimeField>(c: &[impl Into<F> + Clone]) -> DensePolynomial<F> {
    DensePolynomial::from_coefficients_vec(c.iter().rev().map(|e| e.clone().into()).collect())
}

/// Computes the first argument raised to the power of the second argument
/// modulo the third one by means of the left-to-right binary exponentiation
/// method. The second argument is an unsigned integer represented by its
/// 64-bit chunks stored in the little-endian order. If the modulus is 0,
/// then None is returned.
pub fn power_modulo<F: PrimeField>(
    p: &DensePolynomial<F>,
    e: &[u64],
    m: &DensePolynomial<F>,
) -> Option<DensePolynomial<F>> {
    let p = reduced_modulo(p, m)?;
    let mut r = new(&[(m.degree() > 0) as u64]);
    for b in crate::num::reversed_bits(e) {
        r = reduced_modulo(&(&r * &r), m)?;
        if b {
            r = reduced_modulo(&(&r * &p), m)?;
        }
    }
    Some(r)
}

/// Computes the first argument modulo the second.
/// If the modulus is 0, then None is returned.
pub fn reduced_modulo<F: PrimeField>(
    p: &DensePolynomial<F>,
    m: &DensePolynomial<F>,
) -> Option<DensePolynomial<F>> {
    // This check is performed to avoid panics of "divide_with_q_and_r",
    // because for some reason "divide_with_q_and_r" never returns None
    if m.is_zero() {
        return None;
    }
    Some(
        DenseOrSparsePolynomial::from(p)
            .divide_with_q_and_r(&DenseOrSparsePolynomial::from(m))?
            .1,
    )
}
