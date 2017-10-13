/// Oldschool in-place selection sort
pub fn insertion_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::insertion_sort;

    use super::super::tests;

    #[test]
    fn random() {
        let mut answer = tests::random_array();
        answer.sort();
        let mut arr = tests::random_array();
        insertion_sort(&mut arr);
        assert_eq!(&arr, &answer);
    }

    #[test]
    fn nearly_sorted() {
        let mut answer = tests::ascendant_array();
        answer.sort();
        let mut arr = tests::ascendant_array();
        insertion_sort(&mut arr);
        assert_eq!(&arr, &answer);
    }

    #[test]
    fn reverse() {
        let mut answer = tests::decendant_array();
        answer.sort();
        let mut arr = tests::decendant_array();
        insertion_sort(&mut arr);
        assert_eq!(&arr, &answer);
    }
}
