//! Provides auxiliary tools for working with numbers.

/// Computes the ascending list of prime numbers that divide
/// the argument by means of the wheel factorization method.
pub fn prime_divisors(mut n: u32) -> Vec<u32> {
    if n < 2 {
        return vec![];
    }
    let mut p = Vec::with_capacity(n.ilog2() as usize);
    if n % 2 == 0 {
        p.push(2);
        n >>= n.trailing_zeros();
    }
    fn extract(f: u32, m: &mut u32, l: &mut Vec<u32>) {
        l.push(f);
        *m /= f;
        while *m % f == 0 {
            *m /= f;
        }
    }
    if n % 3 == 0 {
        extract(3, &mut n, &mut p);
    }
    fn sqrt(v: u32) -> u32 {
        (v as f64).sqrt().round() as u32
    }
    let (mut d, mut s, mut b) = (5u32, 2u32, sqrt(n));
    while d <= b {
        if n % d == 0 {
            extract(d, &mut n, &mut p);
            b = sqrt(n);
        }
        // At this point, a trial divisor is either 5 or 6k + 1
        // or 6k + 5 for some positive integer k. Therefore, the
        // difference between the (j + 1)-th and the j-th trial
        // divisor is 2, if j is odd, and 4 otherwise. The value
        // of the difference for the next iteration is computed
        // from the current difference value in accordance with
        // the equalities "2 xor 6 = 4" and "4 xor 6 = 2"
        (d, s) = (d + s, s ^ 6);
    }
    if n > 1 {
        p.push(n)
    }
    p
}

/// Creates an iterator over the bits of the argument's big-endian
/// binary representation with leading zeroes removed. The argument
/// is an unsigned integer represented by its 64-bit chunks stored
/// in the little-endian order. If the argument is 0, an empty
/// iterator is returned.
pub fn reversed_bits(n: &[u64]) -> impl Iterator<Item = bool> + '_ {
    n.iter()
        .rev()
        .copied()
        .skip_while(|c| *c == 0)
        .map(|c| c.reverse_bits())
        .flat_map(|c| (0..64).map(move |b| c & (1 << b) != 0))
        .skip_while(|b| !b)
}
