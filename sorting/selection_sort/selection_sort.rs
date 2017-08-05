// Oldschool index-based, in-place selection sort
fn selection_sort(arr: &mut [i32]) {
    // Rust would skip iteration if lower bound >= upper bound.
    // Hence, `arr.len() - 1` is only a skip of last iteration.
    for i in 0..(arr.len() - 1) {
        let mut temp = i;
        for j in (i + 1)..arr.len() {
            if arr[temp] < arr[j] { // Descending order
                temp = j;
            }
        }
        arr.swap(i, temp);
    }
}

fn main() {
    let mut arr0 = [1, 2, 3, 4, 5];
    selection_sort(&mut arr0);
    println!("{:?}", arr0);

    let mut arr1 = [2, 1, 3, 8, 3, 4, 9];
    selection_sort(&mut arr1);
    println!("{:?}", arr1);
}
