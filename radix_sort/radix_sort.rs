/// Radix sort

fn radix_sort(arr: &mut [i32]) {
}

fn main() {
    let mut arr0 = [5, 4, 3, 2, 1];
    radix_sort(&mut arr0);
    println!("{:?}", arr0);

    let mut arr1 = [17, 20, 2, 1, 3, 21, 8, 3, 4, 9];
    radix_sort(&mut arr1);
    println!("{:?}", arr1);
}

