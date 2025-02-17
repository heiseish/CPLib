fn dfs_calc(
    idx: usize,
    nc: &mut Vec<usize>,
    nm: &mut Vec<usize>,
    visited: &mut Vec<bool>,
    mar: &Vec<usize>,
    g: &Vec<Vec<usize>>,
) {
    visited[idx] = true;
    let mut tnc = 1;
    let mut tnm = mar[idx];
    for c in &g[idx] {
        if visited[*c] {
            continue;
        }
        dfs_calc(*c, nc, nm, visited, mar, g);
        tnc += nc[*c];
        tnm += nm[*c];
    }
    nc[idx] = tnc;
    nm[idx] = tnm;
}

fn dfs(
    idx: usize,
    nc: &Vec<usize>,
    nm: &Vec<usize>,
    visited: &mut Vec<bool>,
    mar: &Vec<usize>,
    g: &Vec<Vec<usize>>,
) -> i32 {
    visited[idx] = true;
    let mut res = 0;
    for c in &g[idx] {
        if visited[*c] {
            continue;
        }
        res += nc[*c].abs_diff(nm[*c]) as i32;
        res += dfs(*c, nc, nm, visited, mar, g);
    }
    res
}
