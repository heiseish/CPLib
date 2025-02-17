fn compress(mut a: Vec<i64>, mi: Option<usize>) -> HashMap<i64, usize> {
    let mut mi = mi.unwrap_or(0);
    let mut ret = HashMap::new();
    for b in a.into_iter().collect::<BTreeSet<_>>().into_iter() {
        if ret.contains_key(&b) {
            continue;
        }
        ret.insert(b, mi);
        mi += 1;
    }
    ret
}
