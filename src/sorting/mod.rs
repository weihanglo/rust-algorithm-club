mod bubble_sort;
pub use self::bubble_sort::bubble_sort;
pub use self::bubble_sort::bubble_sort_optimized;

mod insertion_sort;
pub use self::insertion_sort::insertion_sort;

mod selection_sort;
pub use self::selection_sort::selection_sort;

mod shellsort;
pub use self::shellsort::shellsort;
pub use self::shellsort::GAPS;

mod mergesort;
pub use self::mergesort::mergesort;
pub use self::mergesort::mergesort_recursive;

mod heapsort;
pub use self::heapsort::heapsort;

mod quicksort;
pub use self::quicksort::quicksort_hoare;
pub use self::quicksort::quicksort_lomuto;

mod counting_sort;
pub use self::counting_sort::counting_sort;

mod bucket_sort;
pub use self::bucket_sort::bucket_sort;

mod radix_sort;
pub use self::radix_sort::radix_sort;


#[cfg(test)]
pub mod tests {
    pub fn ascendant_array() -> [u8; 8] {
        [1, 2, 3, 2, 5, 8, 7, 9]
    }

    pub fn decendant_array() -> [u8; 9] {
        [23, 9, 15, 8, 5, 5, 3, 1, 2]
    }

    pub fn random_array() -> [u8; 10] {
        [232, 239, 192, 199, 45, 133, 132, 151, 56, 194]
        // use rand;
        // const len: usize = 10;
        // let mut vec = Vec::with_capacity(len);
        // while vec.len() < len {
        //     let x = rand::random::<u8>();
        //     vec.push(x);
        // }
        // let mut arr: [u8; len] = [0; len];
        // arr.copy_from_slice(&vec);
        // arr
    }
}
