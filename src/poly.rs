use ark_ff::{PrimeField, Zero};
use ark_poly::{
    polynomial::univariate::{DenseOrSparsePolynomial, DensePolynomial},
    DenseUVPolynomial, Polynomial,
};

pub fn new<F: PrimeField>(c: &[impl Into<F> + Clone]) -> DensePolynomial<F> {
    DensePolynomial::from_coefficients_vec(c.iter().map(|e| e.clone().into()).collect())
}

pub fn reduced_modulo<F: PrimeField>(
    p: &DensePolynomial<F>,
    m: &DensePolynomial<F>,
) -> Option<DensePolynomial<F>> {
    // This check is to avoid a "divide_with_q_and_r" panic, since
    // for some reason "divide_with_q_and_r" never returns None
    if m.is_zero() {
        return None;
    }
    Some(
        DenseOrSparsePolynomial::from(p)
            .divide_with_q_and_r(&DenseOrSparsePolynomial::from(m))?
            .1,
    )
}

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

pub fn nonmonic_gcd<F: PrimeField>(
    a: &DensePolynomial<F>,
    b: &DensePolynomial<F>,
) -> DensePolynomial<F> {
    let mut a = match reduced_modulo(a, b) {
        Some(r) => r,
        None => return a.clone(),
    };
    let mut b = match reduced_modulo(b, &a) {
        Some(r) => r,
        None => return b.clone(),
    };
    while let Some(r) = reduced_modulo(&a, &b) {
        (a, b) = (b, r);
    }
    a
}
