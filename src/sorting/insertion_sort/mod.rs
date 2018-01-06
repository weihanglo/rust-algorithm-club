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

/// Handmade binary search.
/// You may want to use `std::slice::binary_search` instead.
fn binary_search(arr: &[i32], val: i32) -> Result<usize, usize> {
    let mut size = arr.len();
    if size == 0 {
        return Err(0)
    }

    let mut base = 0usize;
    while size > 1 {
        // mid: [base..size)
        let half = size / 2;
        let mid = base + half;
        if arr[mid] <= val {
            base = mid
        }
        size -= half;
    }

    if arr[base] == val {
        Ok(base)
    } else {
        // Return the expected position in the array.
        Err(base + (arr[base] < val) as usize)
    }
}

/// Binary insertion sort is a insertion sort variant thas utilize binary
/// search to reduce comparisons in a normal insertion sort.
pub fn binary_insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let val = arr[i];
        let mut j = i;
        let pos = match binary_search(&arr[0..j], val) {
            Ok(pos) => pos,
            Err(pos) => pos,
        };
        // Swap all elements until specific position.
        while j > pos {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::tests;

    #[test]
    fn search() {
        // Empty array.
        let arr = [];
        assert_eq!(binary_search(&arr, 0), Err(0));
        let arr = [1];
        assert_eq!(binary_search(&arr, 1), Ok(0));
        let arr = [1,2];
        assert_eq!(binary_search(&arr, 2), Ok(1));
        let arr = [1,2];
        assert_eq!(binary_search(&arr, 3), Err(2));
        let arr = [1,2,3,4];
        assert_eq!(binary_search(&arr, 4), Ok(3));
        let arr = [1,2,4,5];
        assert_eq!(binary_search(&arr, 3), Err(2));
    }
}
