/// Handmade binary search for a sorted sequence.
///
/// This version of binary search does not guarantee retrieval of the leftmost
/// matching position if multiple elements found.
///
/// Reference:
///
/// - [`std::slice::binary_search`][1]
///
/// [1]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
pub fn binary_search<T>(arr: &[T], target: &T) -> Result<usize, usize>
where
    T: PartialOrd,
{
    let mut size = arr.len();
    if size == 0 {
        return Err(0);
    }
    let mut base = 0_usize;

    while size > 1 {
        // mid: [base..size)
        let half = size / 2;
        let mid = base + half;
        if arr[mid] <= *target {
            base = mid
        }
        size -= half;
    }

    if arr[base] == *target {
        Ok(base)
    } else {
        // Return the expected position in the array.
        Err(base + (arr[base] < *target) as usize)
    }
}

#[cfg(test)]
mod base {
    use super::*;

    sorted_no_duplicate_cases!(binary_search);
}
