/// Insertion sort.
pub fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Binary insertion sort.
///
/// Binary insertion sort is a insertion sort variant that utilizes binary
/// search to reduce comparisons in a normal insertion sort.
pub fn binary_insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let val = arr[i];
        let mut j = i;
        let pos = arr[..i].binary_search(&val).unwrap_or_else(|pos| pos);
        // Swap all elements until specific position.
        while j > pos {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod base {
    use super::*;
    base_cases!(insertion_sort);
}

#[cfg(test)]
mod binary_insertion {
    use super::*;
    base_cases!(binary_insertion_sort);
}
