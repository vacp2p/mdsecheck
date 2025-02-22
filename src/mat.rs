//! Provides auxiliary tools for working with matrices.

use ark_ff::Field;

/// Computes the matrix product of the arguments. If the arguments are
/// not matrices for which the product is defined, then None is returned.
pub fn product_matrix<F: Field>(
    a: &[impl AsRef<[F]>],
    b: &[impl AsRef<[F]>],
) -> Option<Vec<Vec<F>>> {
    let k = b.len();
    if (k == 0) || a.is_empty() {
        // Some matrix is empty
        return None;
    }
    let c = b[0].as_ref().len();
    if a.iter().any(|s| s.as_ref().len() != k) || (0..k).any(|y| b[y].as_ref().len() != c) {
        // The arguments are not matrices for which the product is defined
        return None;
    }
    let mut m = Vec::<Vec<F>>::with_capacity(a.len());
    for s in a.iter() {
        let mut l = Vec::<F>::with_capacity(c);
        let s = s.as_ref();
        for x in 0..c {
            l.push((0..k).map(|i| s[i] * b[i].as_ref()[x]).sum());
        }
        m.push(l);
    }
    Some(m)
}

/// Computes the product of the specified matrix and column vector. If the arguments
/// are not a matrix-vector pair for which the product is defined, then None is returned.
pub fn product_vector<F: Field>(a: &[impl AsRef<[F]>], b: &[F]) -> Option<Vec<F>> {
    let k = b.len();
    if (k == 0) || a.is_empty() {
        // Either the matrix or the vector is empty
        return None;
    }
    if a.iter().any(|s| s.as_ref().len() != k) {
        // The arguments are not a matrix-vector pair for which the product is defined
        return None;
    }
    let mut v = Vec::<F>::with_capacity(a.len());
    for s in a.iter() {
        v.push(s.as_ref().iter().zip(b.iter()).map(|(x, y)| *x * y).sum());
    }
    Some(v)
}

/// Computes such a column vector that the product of the specified nonsingular
/// matrix and it is equal to the specified column vector by means of the Gaussian
/// elimination method. If the first argument is not a nonsingular matrix of the
/// height of the specified vector, then None is returned.
pub fn system_solution<F: Field>(a: &[impl AsRef<[F]>], b: &[F]) -> Option<Vec<F>> {
    let n = a.len();
    if n == 0 {
        // The matrix is empty
        return None;
    }
    if b.len() != n {
        // The heights of the matrix and column vector are different
        return None;
    }
    let mut m = Vec::<Vec<F>>::with_capacity(n);
    for (r, v) in a.iter().zip(b.iter()) {
        let r = r.as_ref();
        if r.len() != n {
            // The first argument is not a square matrix
            return None;
        }
        m.push([r, &[*v][..]].concat());
    }
    // Obtaining the row echelon form of the augmented matrix
    for r in 0..n {
        if m[r][r] == F::ZERO {
            if let Some(p) = (r + 1..n).find(|y| m[*y][r] != F::ZERO) {
                m.swap(r, p);
            } else {
                // The matrix is singular
                return None;
            }
        }
        let c = m[r][r];
        m[r][r] = F::ONE;
        for x in r + 1..=n {
            m[r][x] /= c;
        }
        for y in r + 1..n {
            let c = m[y][r];
            m[y][r] = F::ZERO;
            for x in r + 1..=n {
                m[y][x] = m[y][x] - c * m[r][x];
            }
        }
    }
    // Transforming the rightmost column of the row
    // echelon matrix into the sought column vector
    for r in (1..n).rev() {
        for y in 0..r {
            m[y][n] = m[y][n] - m[y][r] * m[r][n];
        }
    }
    Some((0..n).map(|y| m[y][n]).collect())
}
