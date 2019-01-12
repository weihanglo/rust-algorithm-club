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

        #[test]
        fn random() {
            // Similar behavior as `std::slice::binary_search`.
            //
            // Test for
            //
            // - random array lengh
            // - random elements
            // - random target
            use rand;

            for _ in 0..100 {
                let len = rand::random::<usize>() % 500;
                let mut arr: Vec<_> = (0..len).map(|_| rand::random::<i32>() % 2000).collect();
                arr.sort_unstable();
                arr.dedup();

                (0..50).for_each(|_| {
                    let target = rand::random::<i32>() % 4000 - 1000;
                    assert_eq!(arr.binary_search(&target).ok(), $algo(&arr, &target),)
                })
            }
        }
    };
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
            assert_eq!($algo(arr, target), expected, "0 -> Ok(0)");

            let arr = &[0];
            let target = &1;
            let expected = Err(1);
            assert_eq!($algo(arr, target), expected, "1 -> Err(1)");
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
            let arr = &[0, 2, 4, 6, 8, 10, 12, 14, 16, 18];
            let target = &0;
            let expected = Ok(0);
            assert_eq!($algo(arr, target), expected);

            let target = &2;
            let expected = Ok(1);
            assert_eq!($algo(arr, target), expected);

            let target = &4;
            let expected = Ok(2);
            assert_eq!($algo(arr, target), expected);

            let target = &6;
            let expected = Ok(3);
            assert_eq!($algo(arr, target), expected);

            let target = &1;
            let expected = Err(1);
            assert_eq!($algo(arr, target), expected);

            let target = &3;
            let expected = Err(2);
            assert_eq!($algo(arr, target), expected);

            let target = &5;
            let expected = Err(3);
            assert_eq!($algo(arr, target), expected);

            let target = &7;
            let expected = Err(4);
            assert_eq!($algo(arr, target), expected);

            let target = &20;
            let expected = Err(10);
            assert_eq!($algo(arr, target), expected);

            let target = &-1;
            let expected = Err(0);
            assert_eq!($algo(arr, target), expected);
        }

        #[test]
        fn not_found() {
            let arr = &[1, 2, 3];
            let target = &888;
            let expected = Err(3);
            assert_eq!($algo(arr, target), expected);
        }

        #[test]
        fn random() {
            // Same behavior as `std::slice::binary_search`.
            //
            // Test for
            //
            // - random array lengh
            // - random elements
            // - random target
            use rand;

            for _ in 0..100 {
                let len = rand::random::<usize>() % 500;
                let mut arr: Vec<_> = (0..len).map(|_| rand::random::<i32>() % 2000).collect();
                arr.sort_unstable();
                arr.dedup();

                (0..50).for_each(|_| {
                    let target = rand::random::<i32>() % 4000 - 1000;
                    assert_eq!(arr.binary_search(&target), $algo(&arr, &target),)
                })
            }
        }
    };
}
