// elem -> freq
fn get_smallest_value<T: Clone>(m: &BTreeMap<T, i64>, kth: usize) -> Option<T> {
    let mut ki = m.iter();
    let mut f: Option<&T> = None;
    let mut cnt = 0;
    while cnt < kth {
        let (k, v) = ki.next()?;
        f = Some(k);
        for _ in 0..(*v) {
            cnt += 1;
        }
    }
    f.cloned()
}

// elem -> freq
fn get_largest_value<T: Clone>(m: &BTreeMap<T, i64>, kth: usize) -> Option<T> {
    let mut ki = m.iter();
    let mut f: Option<&T> = None;
    let mut cnt = 0;
    while cnt < kth {
        let (k, v) = ki.next_back()?;
        f = Some(k);
        for _ in 0..(*v) {
            cnt += 1;
        }
    }
    f.cloned()
}

fn get_smallest_value<T: Clone>(m: &BTreeSet<T>, kth: usize) -> Option<T> {
    let mut ki = m.iter();
    let mut f: Option<&T> = None;
    let mut cnt = 0;
    while cnt < kth {
        let k = ki.next()?;
        f = Some(k);
    }
    f.cloned()
}

fn get_largest_value<T: Clone>(m: &BTreeSet<T>, kth: usize) -> Option<T> {
    let mut ki = m.iter();
    let mut f: Option<&T> = None;
    let mut cnt = 0;
    while cnt < kth {
        let k = ki.next_back()?;
        f = Some(k);
    }
    f.cloned()
}
