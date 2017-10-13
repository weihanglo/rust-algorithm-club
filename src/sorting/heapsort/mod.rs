/// Heapsort
pub fn heapsort<T: PartialOrd + Copy>(arr: &mut [T]) {
    // Heapify part (build max-heap/min-heap)
    let end = arr.len();
    for start in (0..end).rev() {
        sift_down(arr, start, end - 1);
    }

    // Sorting part
    for end in (1..arr.len()).rev() {
        arr.swap(end, 0);
        sift_down(arr, 0, end - 1);
    }
}

fn sift_down<T: PartialOrd + Copy>(arr: &mut [T], start: usize, end: usize) {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1; // Get the left child
        if child > end {
            break;
        }
        if child + 1 <= end && arr[child] < arr[child + 1] {
            // Right child exists and is greater.
            child += 1;
        }

        if arr[root] < arr[child] {
            // If child is greater than root, swap'em!
            arr.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}
