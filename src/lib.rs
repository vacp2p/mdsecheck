#![doc = include_str!("../README.md")]

use ark_ff::PrimeField;
use ark_poly::{polynomial::univariate::DensePolynomial, DenseUVPolynomial};
use indexmap::IndexSet;
use rand::Rng;

pub mod mat;
pub mod num;
pub mod poly;

/// Creates a random Cauchy square MDS matrix, which has the order specified by the first
/// argument and the entries, which are determined by both the order and the source of
/// randomness specified by the second argument. If the first argument is 0 or the field
/// does not have enough elements for the specified matrix order, then None is returned.
pub fn random_cauchy<F: PrimeField>(n: u32, r: &mut (impl Rng + ?Sized)) -> Option<Vec<Vec<F>>> {
    if (n == 0) || (F::MODULUS < F::BigInt::from(n as u64 * 2)) {
        // The first argument is 0 or the field does not have
        // enough elements for the specified matrix order
        return None;
    }
    let n = n as usize;
    let mut d = IndexSet::with_capacity(2 * n);
    for _ in 0..2 * n {
        while !d.insert(F::rand(r)) {}
    }
    let mut m = Vec::with_capacity(n);
    for y in d.iter().skip(n) {
        let mut v = Vec::with_capacity(n);
        for x in d.iter().take(n) {
            // Since all elements in IndexSet are distinct, for
            // each pair of them the difference is invertible
            v.push((*x - y).inverse().unwrap());
        }
        m.push(v);
    }
    Some(m)
}

/// Computes the largest positive number, which exceeds neither the round unconditional
/// P-SPN security level of the specified MDS matrix, nor the second argument, by means
/// of the MDSECheck method. The matrix is not checked for being MDS, so it should be
/// generated properly, e.g. using the tools the crate provides. If the matrix is not
/// unconditionally P-SPN secure, then None is returned.
pub fn security_level<F: PrimeField>(a: &[impl AsRef<[F]>], l: u32) -> Option<u32> {
    let n = a.len();
    if (n < 2) || (l == 0) {
        // The first argument is not a matrix, which can
        // be used in P-SPN, or the second argument is 0
        return None;
    }
    // Using the Krylov’s method fragment, which successfully
    // computes the minimal polynomial of the matrix, provided
    // that the polynomial is of maximum degree and irreducible,
    // fails if the polynomial is not of maximum degree and may
    // fail in other cases
    let mut m = Vec::<Vec<F>>::with_capacity(n);
    m.push(vec![F::ONE; n]);
    for i in 0..n - 1 {
        m.push(mat::product_vector(a, &m[i])?);
    }
    let b = mat::product_vector(a, &m[n - 1])?;
    for y in 0..n {
        for x in 0..y {
            (m[y][x], m[x][y]) = (m[x][y], m[y][x]);
        }
    }
    // If the function returns at this point, then the Krylov’s method fragment
    // has failed. Consequently, the matrix is not unconditionally P-SPN secure
    let mut s = mat::system_solution(&m, &b)?;
    s.iter_mut().for_each(|e| *e = -*e);
    s.push(F::ONE);
    let c = DensePolynomial::<F>::from_coefficients_vec(s);
    // Checking the irreducibility of the minimal polynomial, which has been found using
    // the Krylov’s method fragment, by means of Algorithm 2.2.9 in the book "Prime Numbers -
    // A Computational Perspective (2nd edn.)" by R. Crandall and C. Pomerance. Some values
    // computed in this step will be used in the next one
    let f = num::prime_divisors(n as u32)
        .into_iter()
        .map(|e| n / e as usize)
        .collect::<IndexSet<usize>>();
    let mut y: Vec<DensePolynomial<F>> = Vec::<DensePolynomial<F>>::with_capacity(f.len());
    let x = poly::new(&[1, 0]);
    let mut r = x.clone();
    for d in 1..=n / 2 {
        r = poly::power_modulo(&r, F::characteristic(), &c)?;
        if !poly::coprimality(&(&r - &x), &c) {
            // The minimal polynomial is not irreducible
            return None;
        }
        if f.contains(&d) {
            // Saving a value useful for the next step
            y.push(r.clone());
        }
    }
    // Checking that the minimal polynomials for the higher powers of the matrix also are
    // of maximum degree and irreducible. This is done by checking that the higher powers
    // of a root of the characteristic polynomial of the matrix do not belong to nontrivial
    // subfields of the splitting field of the characteristic polynomial. Since the minimal
    // polynomial of the matrix is of maximum degree, it equals the characteristic polynomial
    let (mut g, mut h) = (x.clone(), y.clone());
    for i in 2..=l {
        g = poly::reduced_modulo(&(&g * &x), &c)?;
        for (v, u) in h.iter_mut().zip(y.iter()) {
            *v = poly::reduced_modulo(&(&*v * u), &c)?;
            if *v == g {
                // For the current power of the matrix the minimal polynomial is not
                // of maximum degree or not irreducible, so the round unconditional
                // P-SPN security level of the matrix equals the previous exponent
                return Some(i - 1);
            }
        }
    }
    Some(l)
}
