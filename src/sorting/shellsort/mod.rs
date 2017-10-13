/// Marcin Ciura's gap sequence.
pub const GAPS: &[usize] = &[701, 301, 132, 57, 23, 10, 4, 1];

/// Shellsort
pub fn shellsort(arr: &mut [i32]) {
    let len = arr.len();
    for gap in GAPS.iter() {
        let mut i = *gap; // Type of gap is `&usize`. Deference it!
        while i < len {
            let mut j = i;
            while j >= *gap && arr[j - gap] > arr[j] {
                arr.swap(j - *gap, j);
                j -= *gap;
            }
            i += 1;
        }
    }
}
