/// Oldschool index-based, in-place bubble sort
fn bubble_sort(arr: &mut [i32]) {
    let mut swapped = true;
    while swapped { // No swap means array is sorted.
        swapped = false;
        for i in 1..arr.len() {
            if arr[i - 1] < arr[i] {
                arr.swap(i - 1, i);
                swapped = true
            }
        }
    }
}

// Memorize last swapped index to avoid unnecessary check.
fn bubble_sort_optimized(arr: &mut [i32]) {
    let mut new_len: usize;
    let mut len = arr.len();
    loop {
        new_len = 0;
        for i in 1..len {
            if arr[i - 1] < arr[i] {
                arr.swap(i - 1, i);
                new_len = i;
            }
        }
        if new_len == 0 { break; }
        len = new_len;
    }
}

fn main() {
    let mut arr0 = [1, 2, 3, 4, 5];
    bubble_sort(&mut arr0);
    println!("{:?}", arr0);

    let mut arr1 = [2, 1, 3, 8, 3, 4, 9];
    bubble_sort(&mut arr1);
    println!("{:?}", arr1);

    let mut arr2 = [1, 2, 3, 4, 5];
    bubble_sort_optimized(&mut arr2);
    println!("{:?}", arr2);

    let mut arr3 = [2, 1, 3, 8, 3, 4, 9];
    bubble_sort_optimized(&mut arr3);
    println!("{:?}", arr3);
}
