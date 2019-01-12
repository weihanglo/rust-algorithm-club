use crate::sorting::counting_sort;

/// Radix sort for sorting unsigned integers.
///
/// * `arr` - Collection of value to be sorted in place.
pub fn radix_sort(arr: &mut [i32]) {
    // 1. Choose 10 as our radix.
    let radix = 10;
    // 2. Started from least significant digit (rightmost).
    let mut digit = 1;
    // 3. Find the maximum value to determine break point of the loop.
    let max_value = arr // 3
        .iter()
        .max()
        .unwrap_or(&0)
        .clone();
    // 4. Sorting subroutine (use counting sort).
    while digit <= max_value {
        counting_sort(arr, 0, 9, |t| (t / digit % radix) as usize);
        digit *= radix;
    }
}

#[cfg(test)]
mod base {
    use super::*;
    base_cases!(radix_sort);
}
