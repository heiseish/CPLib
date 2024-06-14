mod fft {
    use crate::mint;

    // FIXME
    // fn new() -> Self {
    //     let rank2 = (MOD - 1).trailing_zeros() as usize;
    //     let mut root = [ModInt::new(0); 32];
    //     let mut iroot = [ModInt::new(0); 32];
    //     let mut rate2 = [ModInt::new(0); 30];
    //     let mut irate2 = [ModInt::new(0); 30];
    //     let mut rate3 = [ModInt::new(0); 28];
    //     let mut irate3 = [ModInt::new(0); 28];

    //     root[rank2] = ModInt::new(G).pow((MOD - 1) >> rank2);
    //     iroot[rank2] = root[rank2].inv();

    //     for i in (0..rank2).rev() {
    //         root[i] = root[i + 1] * root[i + 1];
    //         iroot[i] = iroot[i + 1] * iroot[i + 1];
    //     }

    //     let mut prod = ModInt::new(1);
    //     let mut iprod = ModInt::new(1);
    //     for i in 0..=rank2 - 2 {
    //         rate2[i] = root[i + 2] * prod;
    //         irate2[i] = iroot[i + 2] * iprod;
    //         prod *= iroot[i + 2];
    //         iprod *= root[i + 2];
    //     }

    //     prod = ModInt::new(1);
    //     iprod = ModInt::new(1);
    //     for i in 0..=rank2 - 3 {
    //         rate3[i] = root[i + 3] * prod;
    //         irate3[i] = iroot[i + 3] * iprod;
    //         prod *= iroot[i + 3];
    //         iprod *= root[i + 3];
    //     }

    //     Self {
    //         root,
    //         iroot,
    //         rate2,
    //         irate2,
    //         rate3,
    //         irate3,
    //     }
    // }

    const C: usize = 119;
    const ROOT_PW: usize = 1 << 23;
    const G: mint = mint::new(3);
    const ROOT: mint = G.pow(mint::new(C as i64));
    const ROOT_1: mint = ROOT.inv();

    // from https://cp-algorithms.com/algebra/fft.html#number-theoretic-transform
    pub fn fft(a: &mut [mint], invert: bool) {
        let n = a.len();

        let mut j = 0;
        for i in 1..n {
            let mut bit = n >> 1;
            while j & bit != 0 {
                j ^= bit;
                bit >>= 1;
            }
            j ^= bit;
            if i < j {
                a.swap(i, j);
            }
        }

        let mut len = 2;
        while len <= n {
            let mut wlen = if invert { ROOT_1 } else { ROOT };
            let mut i = len;
            while i < ROOT_PW {
                wlen *= wlen;
                i <<= 1;
            }
            for i in (0..n).step_by(len) {
                let mut w = mint::ONE;
                for j in 0..len / 2 {
                    let u = a[i + j];
                    let v = a[i + j + len / 2] * w;
                    a[i + j] = u + v;
                    a[i + j + len / 2] = u - v;
                    w *= wlen;
                }
            }

            len <<= 1;
        }

        if invert {
            let n_1 = mint::new(n as i64).inv();
            for x in a {
                *x *= n_1;
            }
        }
    }

    pub fn convolution(a: &Vec<mint>, b: &Vec<mint>) -> Vec<mint> {
        let mut fa = a.clone();
        let mut fb = b.clone();
        let n = (a.len() + b.len()).next_power_of_two();

        fa.resize(n, mint::ZERO);
        fb.resize(n, mint::ZERO);

        fft(&mut fa, false);
        fft(&mut fb, false);
        for (fai, fbi) in fa.iter_mut().zip(fb) {
            *fai *= fbi;
        }
        fft(&mut fa, true);

        fa
    }
}
