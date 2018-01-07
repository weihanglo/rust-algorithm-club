/// Mergesort.
///
/// - buttom-up (for array-based data structure)
/// - iterative
pub fn mergesort(arr: &mut [i32]) {
    let mut width: usize = 1;
    let mut ret = arr.to_vec(); // for collecting return value
    let len = arr.len();

    while width < len {
        let mut i: usize = 0;
        while i < len {
            let upper = ::std::cmp::min(i + 2 * width, len);
            // width minus 1 for zero-based index
            merge(&arr[i..upper], width - 1, &mut ret[i..upper]);
            i += 2 * width;
        }
        arr.copy_from_slice(&ret[..]);
        width *= 2;
    }
}

/// Recursive merge sort.
///
/// - top-down
/// - recursive
pub fn mergesort_recursive(arr: &mut [i32]) {
    let n = arr.len();
    let mid = n / 2;
    if mid == 0 {
        return;
    }

    mergesort_recursive(&mut arr[..mid]);
    mergesort_recursive(&mut arr[mid..]);

    let mut ret = arr.to_vec();

    // `mid` minus 1 for zero-based index
    merge(arr, mid - 1, &mut ret[..]);

    arr.copy_from_slice(&ret[..]);
}

/// merge helper
fn merge(arr: &[i32], mid: usize, ret: &mut [i32]) {
    let mut left = 0; // head of left pile
    let mut right = mid + 1; // head of right pile
    for i in 0..arr.len() {
        let push_right = right < arr.len() && arr[left] > arr[right];
        if left >= mid + 1 || push_right {
            ret[i] = arr[right];
            right += 1;
        } else {
            ret[i] = arr[left];
            left += 1;
        };
    }
}

#[cfg(test)]
mod base {
    use super::*;
    base_cases!(mergesort);
}

#[cfg(test)]
mod recursive {
    use super::*;
    base_cases!(mergesort_recursive);
}

