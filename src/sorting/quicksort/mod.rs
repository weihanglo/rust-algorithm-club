// -------------------------------------
// Lomuto partition scheme
// -------------------------------------

/// Quicksort with Lomuto parition scheme.
pub fn quicksort(arr: &mut [i32]) {
    let hi = arr.len() as isize - 1;
    quicksort_helper(arr, 0, hi);
}

/// Recursion helper
fn quicksort_helper(arr: &mut [i32], lo: isize, hi: isize) {
    if lo <= hi {
        let pivot = partition(arr, lo, hi);
        quicksort_helper(arr, lo, pivot - 1);
        quicksort_helper(arr, pivot + 1, hi);
    }
}

/// Tail-call opitimized quicksort with Lomuto parition scheme.
pub fn quicksort_optimized(arr: &mut [i32]) {
    let hi = arr.len() as isize - 1;
    quicksort_helper_optimized(arr, 0, hi);
}

/// Tail-call optimized recursion helper.
///
/// Can achieve O(log n) auxiliary space complexity if Rust compiler is
/// implemented with tail-call optimization in the future).
fn quicksort_helper_optimized(arr: &mut [i32], lo: isize, hi: isize) {
    if lo <= hi {
        let pivot = partition(arr, lo, hi);
        if pivot - lo < hi - pivot {                      // 1
          quicksort_helper_optimized(arr, lo, pivot - 1);
          quicksort_helper_optimized(arr, pivot + 1, hi);
        } else {
          quicksort_helper_optimized(arr, pivot + 1, hi);
          quicksort_helper_optimized(arr, lo, pivot - 1);
        }
    }
}

/// Manual tail-call opitimized quicksort with Lomuto parition scheme.
pub fn quicksort_manual_tco(arr: &mut [i32]) {
    let hi = arr.len() as isize - 1;
    quicksort_helper_manual_tco(arr, 0, hi);
}

/// Manual tail-call opitimized recursion helper.
///
/// Can achieve O(log n) auxiliary space complexity without any
/// compiler-optimization of tail-call.
fn quicksort_helper_manual_tco(arr: &mut [i32], mut lo: isize, mut hi: isize) {
    while lo < hi {
        let pivot = partition(arr, lo, hi);
        if pivot - lo < hi - pivot {
            quicksort_helper_manual_tco(arr, lo, pivot - 1);
            lo = pivot + 1;
        } else {
            quicksort_helper_manual_tco(arr, pivot + 1, hi);
            hi = pivot - 1;
        }
    }
}

/// Lomuto partition scheme
///
/// Return index of the pivot.
fn partition(arr: &mut [i32], lo: isize, hi: isize) -> isize {
    // -- Determine the pivot --
    // In Lomuto parition scheme,
    // the latest element is always chosen as the pivot.
    let pivot = arr[hi as usize];
    let mut i = lo;

    // -- Swap elements --
    for j in lo..hi {
        if arr[j as usize] < pivot {
            arr.swap(i as usize, j as usize);
            i += 1;
        }
    }
    // Swap pivot to the middle of two piles.
    arr.swap(i as usize, hi as usize);
    i // Return the final index of the pivot
}

// -------------------------------------
// 3-way partition scheme
// -------------------------------------

/// Quicksort with 3-way parition scheme.
pub fn quicksort_3way(arr: &mut [i32]) {
    let hi = arr.len() as isize - 1;
    quicksort_helper_3way(arr, 0, hi);
}

/// Recursion helper
fn quicksort_helper_3way(arr: &mut [i32], lo: isize, hi: isize) {
    if lo <= hi {
        let (smaller, larger) = partition_3way(arr, lo, hi);
        quicksort_helper_3way(arr, lo, smaller - 1);
        quicksort_helper_3way(arr, larger + 1, hi);
    }
}

/// 3-way paritition scheme
///
/// Return smaller and larger index. (to avoid redundant work on identical elements)
fn partition_3way(arr: &mut [i32], lo: isize, hi: isize) -> (isize, isize) {
    let pivot = arr[hi as usize];
    let mut i = lo;         // smaller
    let mut j = lo;         // equal
    let mut k = hi;         // large

    while j <= k {
        if arr[j as usize] < pivot {
            arr.swap(i as usize, j as usize);
            i += 1;
            j += 1;
        } else if arr[j as usize] > pivot {
            arr.swap(k as usize, j as usize);
            k -= 1;
        } else {
            // No swap when identicial.
            j += 1;
        }
    }

    // Return smaller and larger pointer to avoid iterate duplicate elements.
    (i, k)
}

// -------------------------------------
// Hoare partition scheme
// -------------------------------------

/// Quicksort with Hoare parition scheme
pub fn quicksort_hoare(arr: &mut [i32]) {
    if arr.is_empty() {
        return
    }
    let hi = arr.len() - 1;
    quicksort_helper_hoare(arr, 0, hi);
}

/// Recursion helper
fn quicksort_helper_hoare(arr: &mut [i32], lo: usize, hi: usize) {
    if lo < hi {
        let pivot = partition_hoare(arr, lo, hi);
        quicksort_helper_hoare(arr, lo, pivot);
        quicksort_helper_hoare(arr, pivot + 1, hi);
    }
}

/// Hoare partition scheme
///
/// Return the middle index of the two partitions.
///
/// Note that the return value is not necessarily be the index of the pivot,
/// and the pivot is located somewhere of the first partition.
fn partition_hoare(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let pivot = arr[lo];
    let mut i = lo;
    let mut j = hi;

    loop {
        // Find element >= pivot from leftmost element.
        while arr[i] < pivot {
            i += 1;
        }
        // Find element <= pivot from rightmost element.
        while arr[j] > pivot {
            j -= 1;
        }
        if i >= j {
            return j;
        }
        // Two elements are misplaced, swap them.
        arr.swap(i, j);
        i += 1;
        j -= 1;
    }
}

#[cfg(test)]
mod base {
    use super::*;
    base_cases!(quicksort);
}

#[cfg(test)]
mod optimized {
    use super::*;
    base_cases!(quicksort_optimized);
}

#[cfg(test)]
mod manual_tco {
    use super::*;
    base_cases!(quicksort_manual_tco);
}


#[cfg(test)]
mod three_way {
    use super::*;
    base_cases!(quicksort_3way);
}

#[cfg(test)]
mod hoare {
    use super::*;
    base_cases!(quicksort_hoare);
}
