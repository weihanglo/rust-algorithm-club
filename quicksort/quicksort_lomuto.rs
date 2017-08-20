/// Quicksort with Lomuto parition scheme

fn quicksort(arr: &mut [i32]) {
    let hi = arr.len() - 1;
    quicksort_helper(arr, 0, hi as isize);
}

// Recursion helper
fn quicksort_helper(arr: &mut [i32], lo: isize, hi: isize) {
    if lo <= hi {
        let pivot = partition(arr, lo, hi);
        quicksort_helper(arr, lo, pivot - 1);
        quicksort_helper(arr, pivot + 1, hi);
    }
}

/// Lomuto partition scheme
/// - Return: index of the pivot
fn partition(arr: &mut [i32], lo: isize, hi: isize) -> isize {
    let pivot = arr[hi as usize];
    let mut i = lo;
    // swap elements
    for j in lo..hi {
        if arr[j as usize] < pivot {
            arr.swap(i as usize, j as usize);
            i += 1;
        }
    }
    // swap pivot to the middle of two piles
    arr.swap(i as usize, hi as usize);
    i // return the new pivot
}

fn main() {
    let mut arr0 = [5, 4, 3, 2, 1];
    quicksort(&mut arr0);
    println!("{:?}", arr0);

    let mut arr1 = [17, 20, 2, 1, 3, 21, 8, 3, 4, 9];
    quicksort(&mut arr1);
    println!("{:?}", arr1);
}
