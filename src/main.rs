extern crate rand;

mod sorting;

fn main() {
    let mut arr = [5, 2, 1, 4];
    sorting::bubble_sort(&mut arr);
    println!("{:?}", arr);
}
