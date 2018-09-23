use crate::searching::binary_search;

/// Exponential search. An binary search variant that can perform on
/// unbounded sequences and infinite lists.
///
/// Use [`crate::searching::binary_search`][1] as the underlying algorithm.
///
/// [1]: ./fn.binary_search.html
pub fn exponential_search<T>(arr: &[T], target: &T) -> Result<usize, usize>
    where T: PartialOrd
{
    let size = arr.len();
    if size == 0 {
        return Err(0);
    }

    let mut hi = 1_usize; // Upper bound.
    while hi < size && arr[hi] < *target {
        hi <<= 1;
    }
    let lo = hi >> 1; // Lower bound.

    // Search within [lo..size) or [lo..hi]
    binary_search(&arr[lo..size.min(hi + 1)], target)
        .map(|index| lo + index) // Adjust index offset.
        .map_err(|index| lo + index)
}

#[cfg(test)]
mod base {
    use super::*;

    sorted_no_duplicate_cases!(exponential_search);
}
