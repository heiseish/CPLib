use std::cmp::Ordering;

// So, replacement of std::lower_bound: C++
// This have exact same behaviour as C++ std::lower_bound.
let lower_bound = my_sorted_slice
    .binary_search_by(|element| match element.cmp(&searched_value) {
        // Since we try to find position of first element,
        // we treat all equal values as greater to move left.
        Ordering::Equal => Ordering::Greater,
        ord => ord,
    })
    // Since our comparator never returns `Ordering::Equal`,
    // it would always `Err(idx)`.
    .unwrap_err();

// Another version if we want to have Ok or Err
// like Rust std binary_search.
let (Ok(lower_bound) | Err(lower_bound)) = my_sorted_slice
    .binary_search_by(|element| match element.cmp(&searched_value) {
        Ordering::Equal => Ordering::Greater,
        ord => ord,
    })
    // Check what we found.
    .or_else(|idx| {
        if my_sorted_slice.get(idx) == Some(&searched_value) {
            Ok(idx)
        } else {
            Err(idx)
        }
    });

// And replacement of std::upper_bound: C++
let res: usize = my_sorted_slice
    .binary_search_by(|element| match element.cmp(&searched_value) {
        // Since we try to find position right after searched value,
        // we treat all equal values as less to move right.
        Ordering::Equal => Ordering::Less,
        ord => ord,
    })
    // Since `std::upper_bound` always return position
    // which doesn't point to searched value,
    // we would always get `Err` value from bsearch.
    .unwrap_err();