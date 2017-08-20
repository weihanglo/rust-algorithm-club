/// Mergesort
///
/// - buttom-up (for array-based data structure)
/// - iterative

fn mergesort<T: PartialOrd + Copy>(arr: &mut [T]) {
    let mut width: usize = 1;
    let mut ret = arr.to_vec(); // for collecting return value
    let len = arr.len();

    while width < len {
        let mut i: usize = 0;
        while i < len {
            let upper = std::cmp::min(i + 2 * width, len);
            // width minus 1 for zero-based index
            merge(& arr[i..upper], width - 1, &mut ret[i..upper]);
            i += 2 * width;
        }
        arr.copy_from_slice(&ret[..]);
        width *= 2;
    }
}

/// Oldschool index-based merge sort
///
/// - top-down
/// - recursive
fn mergesort_recursive<T: PartialOrd + Copy>(arr: &mut [T]) {
    let n = arr.len();
    let mid = n / 2;
    if mid == 0 {
        return;
    }

    mergesort_recursive(&mut arr[..mid]);
    mergesort_recursive(&mut arr[mid..]);

    let mut ret = arr.to_vec();

    // `mid` minus 1 for zero-based index
    merge(arr, mid - 1, &mut ret[..]);

    arr.copy_from_slice(&ret[..]);
}

/// merge helper
fn merge<T: PartialOrd + Copy>(arr: & [T], mid: usize, ret: &mut [T]) {
    let mut left = 0;               // head of left pile
    let mut right = mid + 1;        // head of right pile
    for i in 0..arr.len() {
        let push_right = right < arr.len() && arr[left] > arr[right];
        if left >= mid + 1 || push_right {
            ret[i] = arr[right];
            right += 1;
        } else {
            ret[i] = arr[left];
            left += 1;
        };
    }
}


fn main() {
    let mut arr0 = [5, 4, 3, 2, 1];
    mergesort(&mut arr0);
    println!("{:?}", arr0);

    let mut arr1 = [17, 20, 2, 1, 3, 21, 8, 3, 4, 9];
    mergesort(&mut arr1);
    println!("{:?}", arr1);

    let mut arr2 = [5, 4, 3, 2, 1];
    mergesort_recursive(&mut arr2);
    println!("{:?}", arr2);

    let mut arr3 = [17, 20, 2, 1, 3, 21, 8, 3, 4, 9];
    mergesort_recursive(&mut arr3);
    println!("{:?}", arr3);
}
