/// Shellsort

// Marcin Ciura's gap sequence
const GAPS: &[usize] = &[701, 301, 132, 57, 23, 10, 4, 1];

fn shellsort(arr: &mut [i32]) {
    let len = arr.len();
    for gap in GAPS.iter() {
        let mut i = *gap; // Type of gap is `&usize`. Deference it!
        while i < len {
            let mut j = i;
            while j >= *gap && arr[j - gap] > arr[j] {
                arr.swap(j - *gap, j);
                j -= *gap;
            }
            i += 1;
        }
    }
}

fn main() {
    let mut arr0 = [5, 4, 3, 2, 1];
    shellsort(&mut arr0);
    println!("{:?}", arr0);

    let mut arr1 = [17, 20, 2, 1, 3, 21, 8, 3, 4, 9];
    shellsort(&mut arr1);
    println!("{:?}", arr1);
}
