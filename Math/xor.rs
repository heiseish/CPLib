fn number_of_bits_on_bits_off() {
    let mut pre = vec![vec![vec![0i32; 2]; 31]; n + 2];
    let mut suf = vec![vec![vec![0i32; 2]; 31]; n + 2];

    for i in 1..=n {
        let mut msb = 0;
        while 1 << (msb + 1) <= a[i] {
            msb += 1;
        }
        msbs[i] = msb;
    }

    for b in 0..31 {
        for i in 1..=n {
            let t = (a[i] & (1 << b) > 0) as usize;
            for k in 0..2 {
                pre[i][b][k] = (t == k) as i32 + pre[i - 1][b][k ^ t];
            }
        }

        for i in (1..=n).rev() {
            let t = (a[i] & (1 << b) > 0) as usize;
            for k in 0..2 {
                suf[i][b][k] = (t == k) as i32 + suf[i + 1][b][k ^ t];
            }
        }
    }

    // suf[i][b][0] = number of r such that from [i, n), there are XOR of a[i], a[i + 1], .. a[r] has bit b == 0
    // suf[i][b][1] = number of r such that from [i, n), there are XOR of a[i], a[i + 1], .. a[r] has bit b == 1
    // pre[i][b][0] = number of l such that from (0, i], there are XOR of a[1], a[i + 1], .. a[i] has bit b == 0
    // pre[i][b][0] = number of l such that from (0, i], there are XOR of a[1], a[i + 1], .. a[i] has bit b == 1
}
