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
        fn duplicated() {
            let mut arr = [1, 5, 3, 3, 4, 1, 4];
            let res = [1, 1, 3, 3, 4, 4, 5];
            assert(&mut arr, &res);
        }

        #[test]
        fn even_length() {
            let mut arr = [8, 7, 1, 2, 4, 6, 5, 3];
            let res = [1, 2, 3, 4, 5, 6 ,7, 8];
            assert(&mut arr, &res);
        }

        #[test]
        fn odd_length() {
            let mut arr = [7, 1, 2, 4, 6, 5, 3];
            let res = [1, 2, 3, 4, 5, 6, 7];
            assert(&mut arr, &res);
        }
    }
}
