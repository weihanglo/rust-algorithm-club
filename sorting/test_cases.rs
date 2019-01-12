/// Common test cases for any sorting algorithms accepting i32 array as input.
macro_rules! base_cases {
    ($algo:ident) => {
        fn assert(arr: &mut [i32], res: &[i32]) {
            $algo(arr);
            assert_eq!(arr, res);
        }

        #[test]
        fn empty() {
            let mut arr = [];
            let res = [];
            assert(&mut arr, &res);
        }

        #[test]
        fn one_element() {
            let mut arr = [1];
            let res = [1];
            assert(&mut arr, &res);
        }

        #[test]
        fn two_elements() {
            let mut arr = [1, 2];
            let res = [1, 2];
            assert(&mut arr, &res);

            let mut arr = [2, 1];
            let res = [1, 2];
            assert(&mut arr, &res);

            let mut arr = [1, 1];
            let res = [1, 1];
            assert(&mut arr, &res);
        }

        #[test]
        fn three_elements() {
            let mut arr = [1, 1, 2];
            let res = [1, 1, 2];
            assert(&mut arr, &res);

            let mut arr = [2, 1, 1];
            let res = [1, 1, 2];
            assert(&mut arr, &res);

            let mut arr = [1, 2, 1];
            let res = [1, 1, 2];
            assert(&mut arr, &res);

            let mut arr = [1, 2, 3];
            let res = [1, 2, 3];
            assert(&mut arr, &res);

            let mut arr = [1, 3, 2];
            let res = [1, 2, 3];
            assert(&mut arr, &res);

            let mut arr = [2, 1, 3];
            let res = [1, 2, 3];
            assert(&mut arr, &res);

            let mut arr = [2, 3, 1];
            let res = [1, 2, 3];
            assert(&mut arr, &res);

            let mut arr = [3, 1, 2];
            let res = [1, 2, 3];
            assert(&mut arr, &res);

            let mut arr = [3, 2, 1];
            let res = [1, 2, 3];
            assert(&mut arr, &res);
        }

        #[test]
        fn already_sorted() {
            let mut arr = [1, 2, 3, 4, 5];
            let res = [1, 2, 3, 4, 5];
            assert(&mut arr, &res);
        }

        #[test]
        fn reversed() {
            let mut arr = [5, 4, 3, 2, 1];
            let res = [1, 2, 3, 4, 5];
            assert(&mut arr, &res);
        }

        #[test]
        fn all_equal() {
            let mut arr = [1, 1, 1, 1, 1];
            let res = [1, 1, 1, 1, 1];
            assert(&mut arr, &res);
        }

        #[test]
        fn duplicate() {
            let mut arr = [1, 5, 3, 3, 4, 1, 3, 4];
            let res = [1, 1, 3, 3, 3, 4, 4, 5];
            assert(&mut arr, &res);

            let mut arr = [3, 1, 3, 3, 3, 2, 2, 1, 2, 1, 2, 3, 3, 2, 1];
            let res = [1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3];
            assert(&mut arr, &res);
        }

        #[test]
        fn even_length() {
            let mut arr = [8, 7, 1, 2, 4, 6, 5, 3];
            let res = [1, 2, 3, 4, 5, 6, 7, 8];
            assert(&mut arr, &res);
        }

        #[test]
        fn odd_length() {
            let mut arr = [7, 1, 2, 4, 6, 5, 3];
            let res = [1, 2, 3, 4, 5, 6, 7];
            assert(&mut arr, &res);
        }
    };
}

/// Test cases for validate stability of sorting algorithm.
/// The input value is a tuple type of (i32, i32).
///
/// To test with these cases, an algorithm must accept generic input values.
macro_rules! stability_cases {
    ($algo:ident) => {
        /// (key, value)
        fn assert_stability(arr: &mut [(i32, i32)], res: &[(i32, i32)]) {
            $algo(arr);
            assert_eq!(arr, res);
        }

        #[test]
        fn random() {
            let mut arr = [
                (2, 1),
                (1, 1),
                (3, 1),
                (2, 2),
                (2, 3),
                (3, 2),
                (1, 2),
                (1, 3),
                (3, 3),
                (3, 4),
                (1, 4),
                (1, 5),
                (3, 5),
                (2, 4),
                (2, 5),
            ];
            let res = [
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (2, 5),
                (3, 1),
                (3, 2),
                (3, 3),
                (3, 4),
                (3, 5),
            ];
            assert_stability(&mut arr, &res);
        }

        #[test]
        fn interleave() {
            let mut arr = [
                (1, 1),
                (2, 1),
                (3, 1),
                (1, 2),
                (2, 2),
                (3, 2),
                (1, 3),
                (2, 3),
                (3, 3),
                (1, 4),
                (2, 4),
                (3, 4),
                (1, 5),
                (2, 5),
                (3, 5),
            ];
            let res = [
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (2, 5),
                (3, 1),
                (3, 2),
                (3, 3),
                (3, 4),
                (3, 5),
            ];
            assert_stability(&mut arr, &res);
        }
    };
}
