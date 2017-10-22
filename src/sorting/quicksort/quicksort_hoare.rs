/// Quicksort with Hoare parition scheme
pub fn quicksort_hoare(arr: &mut [i32]) {
    let hi = arr.len() - 1;
    quicksort_helper(arr, 0, hi);
}

/// Recursion helper
fn quicksort_helper(arr: &mut [i32], lo: usize, hi: usize) {
    if lo < hi {
        let pivot = partition(arr, lo, hi);
        quicksort_helper(arr, lo, pivot);
        quicksort_helper(arr, pivot + 1, hi);
    }
}

/// Hoare partition scheme
fn partition(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let pivot = arr[lo];
    let mut i = lo;
    let mut j = hi;

    loop {
        while arr[i] < pivot {
            i += 1;
        }
        while arr[j] > pivot {
            j -= 1;
        }
        if i >= j {
            break;
        }
        arr.swap(i, j);
        i += 1;
        j -= 1;
    }
    j // return value
}
