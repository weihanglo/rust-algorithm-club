/// Bubble sort

fn counting_sort(arr: &mut [i32]) {
    let mut swapped = true;
    while swapped { // No swap means array is sorted.
        swapped = false;
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true
            }
        }
    }
}


fn main() {
    let mut arr0 = [5, 4, 3, 2, 1];
    counting_sort(&mut arr0);
    println!("{:?}", arr0);

    let mut arr1 = [17, 20, 2, 1, 3, 21, 8, 3, 4, 9];
    counting_sort(&mut arr1);
    println!("{:?}", arr1);
}
