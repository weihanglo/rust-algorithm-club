/// Oldschool in-place selection sort
fn selection_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    // Rust would skip iteration if lower bound >= upper bound.
    // Hence, `arr.len() - 1` is only a skip of last iteration.
    for i in 0..(arr.len() - 1) {
        let mut temp = i;
        for j in (i + 1)..arr.len() {
            if arr[temp] > arr[j] {
                temp = j;
            }
        }
        arr.swap(i, temp);
    }
}

fn main() {
    let mut arr0 = [5, 4, 3, 2, 1];
    selection_sort(&mut arr0);
    println!("{:?}", arr0);

    let mut arr1 = [17, 20, 2, 1, 3, 21, 8, 3, 4, 9];
    selection_sort(&mut arr1);
    println!("{:?}", arr1);
}
