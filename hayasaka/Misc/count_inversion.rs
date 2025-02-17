// Counting the inversions when merging arrays O(NlogN)
fn count_inv(arr: &mut [i32], left: usize, right: usize) -> usize {
    fn merge(arr: &mut [i32], ll: usize, mm: usize, rr: usize) -> usize {
        let left = arr[ll..=mm].to_vec();
        let right = arr[mm + 1..=rr].to_vec();
        let mut ii = 0;
        let mut jj = 0;
        let mut kk = ll;
        let mut swaps = 0;
        while ii < left.len() && jj < right.len() {
            if left[ii] <= right[jj] {
                arr[kk] = left[ii];
                ii += 1;
            } else {
                arr[kk] = right[jj];
                jj += 1;
                swaps += (mm + 1) - (ll + ii);
            }
            kk += 1;
        }
        while ii < left.len() {
            arr[kk] = left[ii];
            ii += 1;
            kk += 1;
        }
        while jj < right.len() {
            arr[kk] = right[jj];
            jj += 1;
            kk += 1;
        }
        swaps
    }
    let mut cnt = 0;
    if left < right {
        let mid = (left + right) / 2;
        cnt += count_inv(arr, left, mid);
        cnt += count_inv(arr, mid + 1, right);
        cnt += merge(arr, left, mid, right);
    }
    cnt
}
