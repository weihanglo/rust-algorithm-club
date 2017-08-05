// Oldschool index-based, in-place selection sort
fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] < arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn main() {
    let mut arr0 = [1, 2, 3, 4, 5];
    insertion_sort(&mut arr0);
    println!("{:?}", arr0);

    let mut arr1 = [2, 1, 3, 8, 3, 4, 9];
    insertion_sort(&mut arr1);
    println!("{:?}", arr1);
}
