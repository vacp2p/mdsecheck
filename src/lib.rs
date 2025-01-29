use ark_ff::PrimeField;
use ark_poly::{polynomial::univariate::DensePolynomial, DenseUVPolynomial, Polynomial};
use rand::Rng;
use std::collections::HashSet;

pub mod mat;
pub mod num;
pub mod poly;

pub fn random_cauchy<F: PrimeField>(n: u32, r: &mut (impl Rng + ?Sized)) -> Option<Vec<Vec<F>>> {
    if (n == 0) || (F::MODULUS < F::BigInt::from(n as u64 * 2)) {
        return None;
    }
    let n = n as usize;
    let mut d = HashSet::with_capacity(2 * n);
    for _ in 0..2 * n {
        while !d.insert(F::rand(r)) {}
    }
    let mut m = Vec::with_capacity(n);
    for y in d.iter().skip(n) {
        let mut v = Vec::with_capacity(n);
        for x in d.iter().take(n) {
            v.push((*x - y).inverse().unwrap());
        }
        m.push(v);
    }
    Some(m)
}

pub fn secure_rounds<F: PrimeField>(a: &[impl AsRef<[F]>], l: usize) -> Option<usize> {
    if l == 0 {
        return None;
    }
    let n: usize = a.len();
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
    let mut s = mat::unique_solution(&m, &b)?;
    s.iter_mut().for_each(|e| *e = -*e);
    s.push(F::ONE);
    let c = DensePolynomial::<F>::from_coefficients_vec(s);
    let f = num::prime_divisors(n as u32)
        .into_iter()
        .map(|e| n / e as usize)
        .collect::<HashSet<usize>>();
    let mut y: Vec<DensePolynomial<F>> = Vec::<DensePolynomial<F>>::with_capacity(f.len());
    let x = poly::new(&[0u64, 1u64]);
    let mut r = x.clone();
    for d in 1..=n / 2 {
        r = poly::power_modulo(&r, F::characteristic(), &c)?;
        if poly::nonmonic_gcd(&(&r - &x), &c).degree() > 0 {
            return None;
        }
        if f.contains(&d) {
            y.push(r.clone());
        }
    }
    let (mut g, mut h) = (x.clone(), y.clone());
    for i in 2..=l {
        g = poly::reduced_modulo(&(&g * &x), &c)?;
        for (v, u) in h.iter_mut().zip(y.iter()) {
            *v = poly::reduced_modulo(&(&*v * u), &c)?;
            if *v == g {
                return Some(i - 1);
            }
        }
    }
    Some(l)
}
