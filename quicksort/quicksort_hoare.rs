/// Quicksort with Hoare parition scheme

fn quicksort(arr: &mut [i32]) {
    let hi = arr.len() - 1;
    quicksort_helper(arr, 0, hi);
}

// Recursion helper
fn quicksort_helper(arr: &mut [i32], lo: usize, hi: usize) {
    if lo < hi {
        let pivot = partition(arr, lo, hi);
        quicksort_helper(arr, lo, pivot);
        quicksort_helper(arr, pivot + 1, hi);
    }
}

/// Hoare partition scheme
fn partition(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let pivot = arr[lo];
    let mut i = lo;
    let mut j = hi;

    println!("{:?} {:?}", lo, hi);
    loop {
        println!("{:?} {} {} {} {} {}", &arr[lo..hi + 1], lo, hi, pivot, i, j);
        while arr[i] < pivot {
            i += 1;
        }
        while arr[j] > pivot {
            j -= 1;
        }
        if i >= j {
            break
        }
        arr.swap(i, j);
        i += 1;
        j -= 1;
    }
    j // return value
}

fn main() {
    let mut arr0 = [5, 4, 3, 2, 1];
    quicksort(&mut arr0);
    println!("{:?}", arr0);

    let mut arr1 = [17, 20, 2, 1, 3, 21, 8, 3, 4, 9];
    quicksort(&mut arr1);
    println!("{:?}", arr1);
}
