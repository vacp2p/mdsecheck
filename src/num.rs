pub fn prime_divisors(mut n: u32) -> Vec<u32> {
    if n < 2 {
        return vec![];
    }
    let mut p = Vec::with_capacity(n.ilog2() as usize);
    if n & 1 == 0 {
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
        (d, s) = (d + s, s ^ 6);
    }
    if n > 1 {
        p.push(n)
    }
    p
}

pub fn reversed_bits(n: &[u64]) -> impl Iterator<Item = bool> + '_ {
    n.iter()
        .rev()
        .copied()
        .skip_while(|c| *c == 0)
        .map(|c| c.reverse_bits())
        .flat_map(|c| (0..64).map(move |b| c & (1 << b) != 0))
        .skip_while(|b| !b)
}
