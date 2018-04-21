#[cfg(test)]
#[macro_use]
mod test_cases;

mod bubble_sort;
pub use self::bubble_sort::{bubble_sort, bubble_sort_optimized};

mod insertion_sort;
pub use self::insertion_sort::{insertion_sort, binary_insertion_sort};

mod selection_sort;
pub use self::selection_sort::selection_sort;

mod shellsort;
pub use self::shellsort::{shellsort, MARCIN_GAPS};

mod mergesort;
pub use self::mergesort::{mergesort, mergesort_bottom_up};

mod heapsort;
pub use self::heapsort::heapsort;

mod quicksort;
pub use self::quicksort::{
    quicksort,
    quicksort_optimized,
    quicksort_manual_tco,
    quicksort_3way,
    quicksort_hoare, 
};

mod counting_sort;
pub use self::counting_sort::counting_sort;

mod bucket_sort;
pub use self::bucket_sort::bucket_sort;

mod radix_sort;
pub use self::radix_sort::radix_sort;

mod timsort;
pub use self::timsort::timsort;

mod introsort;
pub use self::introsort::introsort;

mod pdqsort;
pub use self::pdqsort::pdqsort;
