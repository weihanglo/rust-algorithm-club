/// Mergesort.
///
/// - Top-down
/// - Recursive
pub fn mergesort(arr: &mut [i32]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    mergesort(&mut arr[..mid]);
    mergesort(&mut arr[mid..]);

    // Create an array to store intermediate result.
    let mut ret = arr.to_vec();

    // Merge the two piles.
    merge(&arr[..mid], &arr[mid..], &mut ret[..]);

    // Copy back the result back to original array.
    arr.copy_from_slice(&ret);
}

/// Mergesort bottom-up version.
///
/// - Buttom-up (for array-based data structure)
/// - Iterative
pub fn mergesort_bottom_up(arr: &mut [i32]) {
    let mut width = 1;
    // Create an array to store intermediate result.
    let mut ret = arr.to_vec();
    let len = arr.len();

    while width < len {
        let mut i = 0;
        while i < len {
            // Check to avoid upper bound and middle index out of bound.
            let upper = ::std::cmp::min(i + 2 * width, len);
            let mid = ::std::cmp::min(i + width, len);

            merge(&arr[i..mid], &arr[mid..upper], &mut ret[i..upper]);

            // Copy the merged result back to original array.
            arr[i..upper].copy_from_slice(&ret[i..upper]);

            // Increase start index to merge next two subsequences.
            i += 2 * width;
        }
        width *= 2;
    }
}

/// Merge helper.
///
/// * `arr1` - Left pile to sort.
/// * `arr2` - Right pile to sort.
/// * `ret` - Result array to return
fn merge(arr1: &[i32], arr2: &[i32], ret: &mut [i32]) {
    let mut left = 0; // Head of left pile.
    let mut right = 0; // Head of right pile.
    let mut index = 0;

    // Compare element and insert back to result array.
    while left < arr1.len() && right < arr2.len() {
        if arr1[left] <= arr2[right] {
            ret[index] = arr1[left];
            index += 1;
            left += 1;
        } else {
            ret[index] = arr2[right];
            index += 1;
            right += 1;
        }
    }

    // Copy the reset elements to returned array.
    // `memcpy` may be more performant than for-loop assignment.
    if left < arr1.len() {
        ret[index..].copy_from_slice(&arr1[left..]);
    }
    if right < arr2.len() {
        ret[index..].copy_from_slice(&arr2[right..]);
    }
}

#[cfg(test)]
mod base {
    use super::*;
    base_cases!(mergesort);
}

#[cfg(test)]
mod bottom_up {
    use super::*;
    base_cases!(mergesort_bottom_up);
}
