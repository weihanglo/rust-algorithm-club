/// Search in sorted sequences by checking the next position based on an
/// linear interpolation of the search key.
///
/// # Parameters
///
/// * `arr`: Slice to search in.
/// * `target`: Object to search for.
///
/// # Notes
///
/// Since interpolations only be applied on numeric types, we choose `i32` to
/// avoid overkilling. If you desire a full functional trait of numeric types,
/// [num][1] crate would meet your needs.
///
/// [1]: https://github.com/rust-num/num
pub fn interpolation_search(arr: &[i32], target: &i32) -> Result<usize, usize> {
    // 1. Handle empty sequence.
    if arr.is_empty() {
        return Err(0);
    }

    // 2. Setup variable storing iteration informaion.
    // hi -> upper bound of search range.
    // lo -> lower bound of search range.
    // interpolant -> position to probe in the sequence
    let mut hi = arr.len() - 1;
    let mut lo = 0_usize;
    let mut interpolant = 0_usize;

    // 3. Main loop to calculate the interpolant.
    loop {
        let lo_val = arr[lo];
        let hi_val = arr[hi];

        // 3.1. Three condition to exit the loop
        //   a. hi and lo flag overlapping -> all elements are scanned.
        //   b. target value is less than the lowest value
        //   c. target value exceeds the highest value
        if hi <= lo || *target < lo_val || *target > hi_val {
            break;
        }

        // 3.2. The linear interpolation part
        let offset = (*target - lo_val) * (hi - lo) as i32 / (hi_val - lo_val);
        interpolant = lo + offset as usize;

        let mid_val = arr[interpolant];

        // 3.3. Comparison between the interpolant and targert value.
        // New boundaries must step one index further to avoid infinite searching.
        if mid_val > *target {
            hi = interpolant - 1;
        } else if mid_val < *target {
            lo = interpolant + 1;
        } else {
            break;
        }
    }

    // 4. Determine whether the returning interpolant equals to target value.
    // `Result::Err` here maps to a position safe to insert while remains ordering.
    if *target > arr[hi] {
        Err(hi + 1)
    } else if *target < arr[lo] {
        Err(lo)
    } else {
        Ok(interpolant)
    }
}

#[cfg(test)]
mod base {
    use super::*;

    sorted_no_duplicate_cases!(interpolation_search);
}
