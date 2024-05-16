use std::ops::{Add, Mul, MulAssign, Sub};

// f64 complex
#[derive(Clone, Copy, Debug)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

impl Complex64 {
    #[inline]
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    #[inline]
    pub fn square_norm(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    #[inline]
    pub fn norm(&self) -> f64 {
        self.square_norm().sqrt()
    }

    #[inline]
    pub fn inverse(&self) -> Complex64 {
        let nrm = self.square_norm();
        Complex64 {
            re: self.re / nrm,
            im: -self.im / nrm,
        }
    }
}

impl Default for Complex64 {
    #[inline]
    fn default() -> Self {
        Self { re: 0.0, im: 0.0 }
    }
}

impl Add<Complex64> for Complex64 {
    type Output = Complex64;

    #[inline]
    fn add(self, other: Complex64) -> Complex64 {
        Complex64 {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Sub<Complex64> for Complex64 {
    type Output = Complex64;

    #[inline]
    fn sub(self, other: Complex64) -> Complex64 {
        Complex64 {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl Mul<Complex64> for Complex64 {
    type Output = Complex64;

    #[inline]
    fn mul(self, other: Complex64) -> Complex64 {
        Complex64 {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl MulAssign<Complex64> for Complex64 {
    #[inline]
    fn mul_assign(&mut self, other: Complex64) {
        let tmp = self.re * other.im + self.im * other.re;
        self.re = self.re * other.re - self.im * other.im;
        self.im = tmp;
    }
}

pub fn fast_fourier_transform_input_permutation(length: usize) -> Vec<usize> {
    let mut result = Vec::new();
    result.reserve_exact(length);
    for i in 0..length {
        result.push(i);
    }
    let mut reverse = 0_usize;
    let mut position = 1_usize;
    while position < length {
        let mut bit = length >> 1;
        while bit & reverse != 0 {
            reverse ^= bit;
            bit >>= 1;
        }
        reverse ^= bit;
        // This is equivalent to adding 1 to a reversed number
        if position < reverse {
            // Only swap each element once
            result.swap(position, reverse);
        }
        position += 1;
    }
    result
}

pub fn fast_fourier_transform(input: &[f64], input_permutation: &[usize]) -> Vec<Complex64> {
    let n = input.len();
    let mut result = Vec::new();
    result.reserve_exact(n);
    for position in input_permutation {
        result.push(Complex64::new(input[*position], 0.0));
    }
    let mut segment_length = 1_usize;
    while segment_length < n {
        segment_length <<= 1;
        let angle: f64 = std::f64::consts::TAU / segment_length as f64;
        let w_len = Complex64::new(angle.cos(), angle.sin());
        for segment_start in (0..n).step_by(segment_length) {
            let mut w = Complex64::new(1.0, 0.0);
            for position in segment_start..(segment_start + segment_length / 2) {
                let a = result[position];
                let b = result[position + segment_length / 2] * w;
                result[position] = a + b;
                result[position + segment_length / 2] = a - b;
                w *= w_len;
            }
        }
    }
    result
}

pub fn inverse_fast_fourier_transform(
    input: &[Complex64],
    input_permutation: &[usize],
) -> Vec<f64> {
    let n = input.len();
    let mut result = Vec::new();
    result.reserve_exact(n);
    for position in input_permutation {
        result.push(input[*position]);
    }
    let mut segment_length = 1_usize;
    while segment_length < n {
        segment_length <<= 1;
        let angle: f64 = -std::f64::consts::TAU / segment_length as f64;
        let w_len = Complex64::new(angle.cos(), angle.sin());
        for segment_start in (0..n).step_by(segment_length) {
            let mut w = Complex64::new(1.0, 0.0);
            for position in segment_start..(segment_start + segment_length / 2) {
                let a = result[position];
                let b = result[position + segment_length / 2] * w;
                result[position] = a + b;
                result[position + segment_length / 2] = a - b;
                w *= w_len;
            }
        }
    }
    let scale = 1.0 / n as f64;
    result.iter().map(|x| x.re * scale).collect()
}

pub struct FFTInfo<const MOD: u32, const G: u32> {
    root: [ModInt<MOD>; 32],
    iroot: [ModInt<MOD>; 32],
    rate2: [ModInt<MOD>; 30],
    irate2: [ModInt<MOD>; 30],
    rate3: [ModInt<MOD>; 28],
    irate3: [ModInt<MOD>; 28],
}

impl<const MOD: u32, const G: u32> FFTInfo<MOD, G> {
    fn new() -> Self {
        let rank2 = (MOD - 1).trailing_zeros() as usize;
        let mut root = [ModInt::new(0); 32];
        let mut iroot = [ModInt::new(0); 32];
        let mut rate2 = [ModInt::new(0); 30];
        let mut irate2 = [ModInt::new(0); 30];
        let mut rate3 = [ModInt::new(0); 28];
        let mut irate3 = [ModInt::new(0); 28];

        root[rank2] = ModInt::new(G).pow((MOD - 1) >> rank2);
        iroot[rank2] = root[rank2].inv();

        for i in (0..rank2).rev() {
            root[i] = root[i + 1] * root[i + 1];
            iroot[i] = iroot[i + 1] * iroot[i + 1];
        }

        let mut prod = ModInt::new(1);
        let mut iprod = ModInt::new(1);
        for i in 0..=rank2 - 2 {
            rate2[i] = root[i + 2] * prod;
            irate2[i] = iroot[i + 2] * iprod;
            prod *= iroot[i + 2];
            iprod *= root[i + 2];
        }

        prod = ModInt::new(1);
        iprod = ModInt::new(1);
        for i in 0..=rank2 - 3 {
            rate3[i] = root[i + 3] * prod;
            irate3[i] = iroot[i + 3] * iprod;
            prod *= iroot[i + 3];
            iprod *= root[i + 3];
        }

        Self {
            root,
            iroot,
            rate2,
            irate2,
            rate3,
            irate3,
        }
    }
}

fn butterfly<M: ModIntTrait>(a: &mut [M]) {
    let n = a.len();
    let h = n.trailing_zeros() as usize;

    let info = fft_info::<M>::new();

    let mut len = 0; // a[i, i+(n>>len), i+2*(n>>len), ..] is transformed
    while len < h {
        if h - len == 1 {
            let p = 1 << (h - len - 1);
            let mut rot = M::new(1);
            for s in 0..(1 << len) {
                let offset = s << (h - len);
                for i in 0..p {
                    let l = a[i + offset];
                    let r = a[i + offset + p] * rot;
                    a[i + offset] = l + r;
                    a[i + offset + p] = l - r;
                }
                if s + 1 != (1 << len) {
                    rot *= info.rate2[(~s).trailing_zeros() as usize];
                }
            }
            len += 1;
        } else {
            // 4-base
            let p = 1 << (h - len - 2);
            let mut rot = M::new(1);
            let imag = info.root[2];
            for s in 0..(1 << len) {
                let rot2 = rot * rot;
                let rot3 = rot2 * rot;
                let offset = s << (h - len);
                for i in 0..p {
                    let mod2 = (M::modulus() as u64).saturating_mul(M::modulus() as u64);
                    let a0 = a[i + offset].val();
                    let a1 = a[i + offset + p].val().saturating_mul(rot.val());
                    let a2 = a[i + offset + 2 * p].val().saturating_mul(rot2.val());
                    let a3 = a[i + offset + 3 * p].val().saturating_mul(rot3.val());
                    let a1na3imag = (a1 + mod2 - a3).saturating_mul(imag.val());
                    let na2 = mod2 - a2;
                    a[i + offset] = a0 + a2 + a1 + a3;
                    a[i + offset + 1 * p] = a0 + a2 + (2 * mod2 - (a1 + a3));
                    a[i + offset + 2 * p] = a0 + na2 + a1na3imag;
                    a[i + offset + 3 * p] = a0 + na2 + (mod2 - a1na3imag);
                }
                if s + 1 != (1 << len) {
                    rot *= info.rate3[(~s).trailing_zeros() as usize];
                }
            }
            len += 2;
        }
    }
}

fn butterfly_inv<M: ModIntTrait>(a: &mut [M]) {
    let n = a.len();
    let h = n.trailing_zeros() as usize;

    let info = fft_info::<M>::new();

    let mut len = h; // a[i, i+(n>>len), i+2*(n>>len), ..] is transformed
    while len > 0 {
        if len == 1 {
            let p = 1 << (h - len);
            let mut irot = M::new(1);
            for s in 0..(1 << (len - 1)) {
                let offset = s << (h - len + 1);
                for i in 0..p {
                    let l = a[i + offset];
                    let r = a[i + offset + p];
                    a[i + offset] = l + r;
                    a[i + offset + p] = (M::modulus() + l.val().saturating_sub(r.val()))
                        .saturating_mul(irot.val());
                }
                if s + 1 != (1 << (len - 1)) {
                    irot *= info.irate2[(~s).trailing_zeros() as usize];
                }
            }
            len -= 1;
        } else {
            // 4-base
            let p = 1 << (h - len);
            let mut irot = M::new(1);
            let iimag = info.iroot[2];
            for s in 0..(1 << (len - 2)) {
                let irot2 = irot * irot;
                let irot3 = irot2 * irot;
                let offset = s << (h - len + 2);
                for i in 0..p {
                    let a0 = a[i + offset + 0 * p].val();
                    let a1 = a[i + offset + 1 * p].val();
                    let a2 = a[i + offset + 2 * p].val();
                    let a3 = a[i + offset + 3 * p].val();

                    let a2na3iimag = ((M::modulus() + a2.saturating_sub(a3)).saturating_mul(
                        iimag.val(),
                    ));

                    a[i + offset] = a0 + a1 + a2 + a3;
                    a[i + offset + 1 * p] =
                        (a0 + (M::modulus() - a1) + a2na3iimag).saturating_mul(irot.val());
                    a[i + offset + 2 * p] = (a0
                        + a1
                        + (M::modulus() - a2)
                        + (M::modulus() - a3))
                    .saturating_mul(irot2.val());
                    a[i + offset + 3 * p] = (a0
                        + (M::modulus() - a1)
                        + (M::modulus() - a2na3iimag))
                    .saturating_mul(irot3.val());
                }
                if s + 1 != (1 << (len - 2)) {
                    irot *= info.irate3[(~s).trailing_zeros() as usize];
                }
            }
            len -= 2;
        }
    }
}

pub fn convolution_fft<const MOD: u32>(
    mut a: Vec<ModInt<MOD>>,
    mut b: Vec<ModInt<MOD>>,
) -> Vec<ModInt<MOD>> {
    let n = a.len();
    let m = b.len();
    let z = bit_ceil((n + m - 1) as u32) as usize;
    a.resize(z, ModInt::new(0));
    butterfly(&mut a);
    b.resize(z, ModInt::new(0));
    butterfly(&mut b);
    for i in 0..z {
        a[i] = a[i] * b[i];
    }
    butterfly_inv(&mut a);
    a.resize(n + m - 1, ModInt::new(0));
    let iz = ModInt::new(z as u32).inv();
    for i in 0..n + m - 1 {
        a[i] = a[i] * iz;
    }
    a
}

pub fn convolution<const MOD: u32>(a: Vec<ModInt<MOD>>, b: Vec<ModInt<MOD>>) -> Vec<ModInt<MOD>> {
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return vec![];
    }

    let z = bit_ceil((n + m - 1) as u32) as u32;
    assert!((MOD - 1) % z == 0);

    if cmp::min(n, m) <= 60 {
        return convolution_naive(a, b);
    }
    convolution_fft(a, b)
}

fn bit_ceil(n: u32) -> u32 {
    32 - n.leading_zeros()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn almost_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }

    const EPSILON: f64 = 1e-6;

    #[test]
    fn small_polynomial_returns_self() {
        let polynomial = vec![1.0f64, 1.0, 0.0, 2.5];
        let permutation = fast_fourier_transform_input_permutation(polynomial.len());
        let fft = fast_fourier_transform(&polynomial, &permutation);
        let ifft = inverse_fast_fourier_transform(&fft, &permutation);
        for (x, y) in ifft.iter().zip(polynomial.iter()) {
            assert!(almost_equal(*x, *y, EPSILON));
        }
    }

    #[test]
    fn square_small_polynomial() {
        let mut polynomial = vec![1.0f64, 1.0, 0.0, 2.0];
        polynomial.append(&mut vec![0.0; 4]);
        let permutation = fast_fourier_transform_input_permutation(polynomial.len());
        let mut fft = fast_fourier_transform(&polynomial, &permutation);
        fft.iter_mut().for_each(|num| *num *= *num);
        let ifft = inverse_fast_fourier_transform(&fft, &permutation);
        let expected = [1.0, 2.0, 1.0, 4.0, 4.0, 0.0, 4.0, 0.0, 0.0];
        for (x, y) in ifft.iter().zip(expected.iter()) {
            assert!(almost_equal(*x, *y, EPSILON));
        }
    }

    #[test]
    #[ignore]
    fn square_big_polynomial() {
        // This test case takes ~1050ms on my machine in unoptimized mode,
        // but it takes ~70ms in release mode.
        let n = 1 << 17; // ~100_000
        let mut polynomial = vec![1.0f64; n];
        polynomial.append(&mut vec![0.0f64; n]);
        let permutation = fast_fourier_transform_input_permutation(polynomial.len());
        let mut fft = fast_fourier_transform(&polynomial, &permutation);
        fft.iter_mut().for_each(|num| *num *= *num);
        let ifft = inverse_fast_fourier_transform(&fft, &permutation);
        let expected = (0..((n << 1) - 1)).map(|i| std::cmp::min(i + 1, (n << 1) - 1 - i) as f64);
        for (&x, y) in ifft.iter().zip(expected) {
            assert!(almost_equal(x, y, EPSILON));
        }
    }
}
