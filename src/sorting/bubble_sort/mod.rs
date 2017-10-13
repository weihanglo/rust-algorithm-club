/// Bubble sort
pub fn bubble_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    let mut swapped = true;
    while swapped {
        // No swap means array is sorted.
        swapped = false;
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true
            }
        }
    }
}

/// Memorize last swapped index to avoid unnecessary check.
pub fn bubble_sort_optimized<T: PartialOrd + Copy>(arr: &mut [T]) {
    let mut new_len: usize;
    let mut len = arr.len();
    loop {
        new_len = 0;
        for i in 1..len {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                new_len = i;
            }
        }
        if new_len == 0 {
            break;
        }
        len = new_len;
    }
}
