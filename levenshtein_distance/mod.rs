use std::cmp;

/// Calculate Levenshtein distance for two strings.
///
/// Returns a minimum number of edits to transform from source to target string.
///
/// Levenshtein distance accepts three edit operations: insertion, deletion,
/// and substitution.
///
/// References:
///
/// - [Levenshtein distance in Cargo][1]
/// - [Ilia Schelokov: Optimizing loop heavy Rust code][2]
///
/// [1]: https://github.com/rust-lang/cargo/blob/7d7fe6797ad07f313706380d251796702272b150/src/cargo/util/lev_distance.rs
/// [2]: https://thaumant.me/optimizing-loop-heavy-rust/
// ANCHOR: lev_dist
pub fn levenshtein_distance(source: &str, target: &str) -> usize {
    // 1
    if source.is_empty() {
        return target.len();
    }
    if target.is_empty() {
        return source.len();
    }

    // 2
    let mut distances = (0..=target.chars().count()).collect::<Vec<_>>();

    for (i, ch1) in source.chars().enumerate() {
        let mut sub = i; // 3
        distances[0] = sub + 1; // 4
        for (j, ch2) in target.chars().enumerate() {
            let dist = cmp::min(
                // 5
                cmp::min(
                    distances[j],     // insert
                    distances[j + 1], // delete
                ) + 1,
                sub + (ch1 != ch2) as usize, // substitute
            );

            sub = distances[j + 1]; // 6
            distances[j + 1] = dist; // 7
        }
    }

    *distances.last().unwrap() // 8
}
// ANCHOR_END: lev_dist

/// Naïvely calculate Levenshtein distance using the whole distance matrix to
/// store information for all substrings.
// ANCHOR: naive
pub fn levenshtein_distance_naive(source: &str, target: &str) -> usize {
    if source.is_empty() {
        return target.len();
    }
    if target.is_empty() {
        return source.len();
    }

    // ANCHOR: naive_init
    let source_count = source.chars().count(); // 1
    let target_count = target.chars().count();

    let mut distances = vec![vec![0; target_count + 1]; source_count + 1]; // 2

    // 3
    for i in 1..=source_count {
        distances[i][0] = i;
    }

    for j in 1..=target_count {
        distances[0][j] = j;
    }
    // ANCHOR_END: naive_init

    // ANCHOR: naive_calc
    for (i, ch1) in source.chars().enumerate() {
        for (j, ch2) in target.chars().enumerate() {
            let ins = distances[i + 1][j] + 1; // 1
            let del = distances[i][j + 1] + 1; // 2
            let sub = distances[i][j] + (ch1 != ch2) as usize; // 3
            distances[i + 1][j + 1] = cmp::min(cmp::min(ins, del), sub); // 4
        }
    }

    // 5
    *distances.last().and_then(|d| d.last()).unwrap()
    // ANCHOR_END: naive_calc
}
// ANCHOR_END: naive

#[cfg(test)]
mod base {
    use super::*;

    fn test_equality(f: impl Fn(&str, &str) -> usize) {
        let cases = ["", "r", "ru", "rus", "rust"];
        for &s in &cases {
            assert_eq!(f(s, s), 0);
        }
    }

    fn test_insertion(f: impl Fn(&str, &str) -> usize) {
        let cases = [
            (1, "rustalgo", "*rustalgo"),
            (2, "rustalgo", "**rustalgo"),
            (3, "rustalgo", "***rustalgo"),
            (1, "rustalgo", "rust*algo"),
            (2, "rustalgo", "rust**algo"),
            (3, "rustalgo", "rust***algo"),
            (1, "rustalgo", "rustalgo*"),
            (2, "rustalgo", "rustalgo**"),
            (3, "rustalgo", "rustalgo***"),
            (2, "rustalgo", "*r*ustalgo"),
            (3, "rustalgo", "*r*u*stalgo"),
            (4, "rustalgo", "*ru**stalgo*"),
        ];
        for &(dist, s1, s2) in &cases {
            assert_eq!(f(s1, s2), dist);
            assert_eq!(f(s2, s1), dist);
        }
    }

    fn test_deletion(f: impl Fn(&str, &str) -> usize) {
        let cases = [
            (1, "rustalgo", "ustalgo"),
            (2, "rustalgo", "stalgo"),
            (3, "rustalgo", "talgo"),
            (1, "rustalgo", "rustalg"),
            (2, "rustalgo", "rustal"),
            (3, "rustalgo", "rusta"),
            (2, "rustalgo", "utalgo"),
            (3, "rustalgo", "rstag"),
            (8, "rustalgo", ""),
        ];
        for &(dist, s1, s2) in &cases {
            assert_eq!(f(s1, s2), dist);
            assert_eq!(f(s2, s1), dist);
        }
    }

    fn test_substitution(f: impl Fn(&str, &str) -> usize) {
        let cases = [
            (1, "rustalgo", "*ustalgo"),
            (2, "rustalgo", "**stalgo"),
            (3, "rustalgo", "***talgo"),
            (1, "rustalgo", "rusta*go"),
            (2, "rustalgo", "rusta**o"),
            (3, "rustalgo", "rusta***"),
            (3, "rustalgo", "*u*t*lgo"),
            (4, "rustalgo", "r**t*lg*"),
            (8, "rustalgo", "********"),
        ];
        for &(dist, s1, s2) in &cases {
            assert_eq!(f(s1, s2), dist);
            assert_eq!(f(s2, s1), dist);
        }
    }

    fn test_mixed(f: impl Fn(&str, &str) -> usize) {
        let sample = [
            (8, "", "rustalgo"),
            (4, "rustalgo", "**ruslgo"),
            (3, "kitten", "sitting"),
            (3, "saturday", "sunday"),
            (3, "台灣國語", "閩南語"),
            (7, "⭕️❌肺炎染", "嚴重⭕️傳染性💩肺炎"),
        ];
        for &(dist, s1, s2) in &sample {
            assert_eq!(f(s1, s2), dist);
            assert_eq!(f(s2, s1), dist);
        }
    }

    // optimized

    #[test]
    fn equality() {
        test_equality(levenshtein_distance);
    }

    #[test]
    fn insertion() {
        test_insertion(levenshtein_distance);
    }

    #[test]
    fn deletion() {
        test_deletion(levenshtein_distance);
    }

    #[test]
    fn substitution() {
        test_substitution(levenshtein_distance);
    }

    #[test]
    fn mixed() {
        test_mixed(levenshtein_distance);
    }

    // naive implementation

    #[test]
    fn equality_naive() {
        test_equality(levenshtein_distance_naive);
    }

    #[test]
    fn insertion_naive() {
        test_insertion(levenshtein_distance_naive);
    }

    #[test]
    fn deletion_naive() {
        test_deletion(levenshtein_distance_naive);
    }

    #[test]
    fn substitution_naive() {
        test_substitution(levenshtein_distance_naive);
    }

    #[test]
    fn mixed_naive() {
        test_mixed(levenshtein_distance_naive);
    }
}
