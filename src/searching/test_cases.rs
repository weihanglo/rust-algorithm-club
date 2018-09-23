/// Test cases for searching algorithms accepting an arbitrary i32 array.
macro_rules! base_cases {
    ($algo:ident) => {
        #[test]
        fn empty() {
            let arr = &[];
            let target = &0;
            let expected = None;
            assert_eq!($algo(arr, target), expected);
        }

        #[test]
        fn one_element() {
            let arr = &[0];
            let target = &0;
            let expected = Some(0);
            assert_eq!($algo(arr, target), expected);
        }

        #[test]
        fn two_elements() {
            let arr = &[0, 1];
            let target = &0;
            let expected = Some(0);
            assert_eq!($algo(arr, target), expected);

            let arr = &[0, 1];
            let target = &1;
            let expected = Some(1);
            assert_eq!($algo(arr, target), expected);

            let arr = &[1, 1];
            let target = &1;
            let expected = Some(0);
            assert_eq!($algo(arr, target), expected);
        }

        #[test]
        fn three_elements() {
            let arr = &[0, 1, 2];
            let target = &1;
            let expected = Some(1);
            assert_eq!($algo(arr, target), expected);

            let arr = &[0, 1, 2];
            let target = &2;
            let expected = Some(2);
            assert_eq!($algo(arr, target), expected);
        }

        #[test]
        fn duplicate() {
            let arr = &[1, 5, 3, 3, 4];
            let target = &3;
            let expected = Some(2);
            assert_eq!($algo(arr, target), expected);
        }

        #[test]
        fn not_found() {
            let arr = &[1, 2, 3];
            let target = &888;
            let expected = None;
            assert_eq!($algo(arr, target), expected);
        }
    }
}

/// Test cases for binary search algorithm and its variants.
/// Accepting a sorted i32 array without any duplicate elements.
macro_rules! sorted_no_duplicate_cases {
    ($algo:ident) => {
        #[test]
        fn empty() {
            let arr = &[];
            let target = &0;
            let expected = Err(0);
            assert_eq!($algo(arr, target), expected);
        }

        #[test]
        fn one_element() {
            let arr = &[0];
            let target = &0;
            let expected = Ok(0);
            assert_eq!($algo(arr, target), expected);

            let arr = &[0];
            let target = &1;
            let expected = Err(1);
            assert_eq!($algo(arr, target), expected);
        }

        #[test]
        fn two_elements() {
            let arr = &[0, 2];
            let target = &0;
            let expected = Ok(0);
            assert_eq!($algo(arr, target), expected);

            let target = &1;
            let expected = Err(1);
            assert_eq!($algo(arr, target), expected);

            let target = &2;
            let expected = Ok(1);
            assert_eq!($algo(arr, target), expected);
        }

        #[test]
        fn multiple() {
            let arr = &[
                0, 1, 2, 3, 4,
                5, 6, 7, 8, 9,
            ];
            let target = &0;
            let expected = Ok(0);
            assert_eq!($algo(arr, target), expected);

            let target = &1;
            let expected = Ok(1);
            assert_eq!($algo(arr, target), expected);

            let target = &2;
            let expected = Ok(2);
            assert_eq!($algo(arr, target), expected);

            let target = &3;
            let expected = Ok(3);
            assert_eq!($algo(arr, target), expected);

            let target = &10;
            let expected = Err(10);
            assert_eq!($algo(arr, target), expected);
        }

        #[test]
        fn not_found() {
            let arr = &[1, 2, 3];
            let target = &888;
            let expected = Err(3);
            assert_eq!($algo(arr, target), expected);
        }
    }
}
