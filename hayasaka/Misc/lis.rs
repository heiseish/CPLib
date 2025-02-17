fn lis(a: &[i32]) -> Vec<usize> {
    let n = a.len();
    let INF = a.iter().max().unwrap() + 1;
    let mut d = vec![INF; n];
    let mut idx = vec![0; n]; // LIS if last element is at i-th

    for i in 0..n {
        let l = d
            .binary_search_by(|element| match element.cmp(&a[i]) {
                // Since we try to find position of first element,
                // we treat all equal values as greater to move left.
                Ordering::Equal => Ordering::Greater,
                ord => ord,
            })
            // Since our comparator never returns `Ordering::Equal`,
            // it would always `Err(idx)`.
            .unwrap_err();
        d[l] = a[i];
        idx[i] = l + 1;
    }
    // let lis = idx.iter().max().unwrap().clone();
    idx
}
