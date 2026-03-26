#![allow(dead_code, unused_variables)]

fn main() {
    println!("Run `cargo test` to check your solutions!");
}

// =============================================================================
// Easy
// =============================================================================

/// Problem 1 (Easy): Sum of a List
///
/// Given a slice of integers, use an iterator chain to compute the sum of all elements.
///
/// Input:   &[1, 2, 3, 4, 5]
/// Output:  15
///
/// Hints: .iter(), .sum()
/// Statements needed: 1
fn sum_of_list(numbers: &[i32]) -> i32 {
    todo!()
}

/// Problem 2 (Easy): Filter Even Numbers
///
/// Given a slice of integers, use an iterator chain to collect only the even numbers
/// into a new vector.
///
/// Input:   &[1, 2, 3, 4, 5, 6, 7, 8]
/// Output:  vec![2, 4, 6, 8]
///
/// Statements needed: 1
fn filter_even(numbers: &[i32]) -> Vec<i32> {
    todo!()
}

/// Problem 3 (Easy): Uppercase All Words
///
/// Given a slice of string slices, use an iterator chain to convert every word to uppercase
/// and collect the results into a new Vec<String>.
///
/// Input:   &["hello", "world", "rust"]
/// Output:  vec!["HELLO", "WORLD", "RUST"]
///
/// Hints: .iter(), .map(), .to_uppercase(), .collect()
/// Statements needed: 1
fn uppercase_all(words: &[&str]) -> Vec<String> {
    todo!()
}

/// Problem 4 (Easy): Count Negative Numbers
///
/// Given a slice of integers, use an iterator chain to count how many numbers are negative.
///
/// Input:   &[3, -1, 4, -1, -5, 9, 2, -6]
/// Output:  4
///
/// Statements needed: 1
fn count_negatives(numbers: &[i32]) -> usize {
    todo!()
}

// =============================================================================
// Medium
// =============================================================================

/// Problem 5 (Medium): Double and Filter
///
/// Given a slice of integers, use an iterator chain to double every number and then keep
/// only those whose doubled value is greater than 5. Collect the results into a new vector.
///
/// Input:   &[1, 2, 3, 4, 5]
/// Output:  vec![6, 8, 10]
///
/// Hints: .iter(), .map(), .filter(), .collect()
/// Statements needed: 1
fn double_and_filter(numbers: &[i32]) -> Vec<i32> {
    todo!()
}

/// Problem 6 (Medium): Longest Word
///
/// Given a slice of string slices, use an iterator chain to find the longest word.
/// Return None if the slice is empty.
///
/// Input:   &["apple", "fig", "banana", "kiwi"]
/// Output:  Some("banana".to_string())
///
/// Statements needed: 1
fn longest_word(words: &[&str]) -> Option<String> {
    todo!()
}

/// Problem 7 (Medium): First Word Starting With a Given Letter
///
/// Given a slice of string slices and a target character, use an iterator chain to find the
/// first word that starts with that character. Return None if no word matches.
///
/// Input:   &["peach", "apricot", "plum", "avocado", "pear"], 'a'
/// Output:  Some("apricot".to_string())
///
/// Hints: .iter(), .find(), .starts_with()
/// Statements needed: 1
fn first_word_starting_with(words: &[&str], letter: char) -> Option<String> {
    todo!()
}

/// Problem 8 (Medium): Zip and Sum Pairs
///
/// Given two slices of integers of equal length, use an iterator chain to pair up elements
/// at the same index and collect their sums into a new vector.
///
/// Input:   &[1, 2, 3, 4], &[10, 20, 30, 40]
/// Output:  vec![11, 22, 33, 44]
///
/// Statements needed: 1
fn zip_and_sum(a: &[i32], b: &[i32]) -> Vec<i32> {
    todo!()
}

// =============================================================================
// Hard
// =============================================================================

/// Problem 9 (Hard): Count Words Longer Than Average
///
/// Given a slice of words, use two chained iterator statements — one to compute the average
/// word length (as f64), and one to count how many words are longer than that average.
///
/// Input:   &["cat", "elephant", "ox", "butterfly", "ant", "rhinoceros"]
/// Output:  3
///
/// Hints: .iter(), .map(), .sum::<usize>(), .filter(), .count()
///        Cast lengths to f64 with `as f64` when computing the average.
/// Statements needed: 2
fn count_words_longer_than_average(words: &[&str]) -> usize {
    todo!()
}

/// Problem 10 (Hard): Second Largest Number
///
/// Given a slice of integers, use an iterator chain with .fold() to find the second largest
/// number without sorting.
///
/// Input:   &[4, 1, 7, 3, 9, 2, 8]
/// Output:  8
///
/// Statements needed: 1
fn second_largest(numbers: &[i32]) -> i32 {
    todo!()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs `$call`, asserting the result equals `$expected`.
    ///
    /// If the function panics with `todo!()` ("not yet implemented"), the test is quietly
    /// skipped so you can run the full suite while only a few problems are done.
    ///
    /// Any *other* panic (e.g. an index-out-of-bounds bug) will still fail the test normally.
    macro_rules! test_problem {
        ($call:expr, $expected:expr) => {{
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $call));
            match result {
                Err(e) => {
                    let is_todo = e
                        .downcast_ref::<&str>()
                        .map(|s| s.contains("not yet implemented"))
                        .unwrap_or(false);
                    if is_todo {
                        eprintln!("[skipped — not yet implemented]");
                    } else {
                        std::panic::resume_unwind(e);
                    }
                }
                Ok(actual) => assert_eq!(actual, $expected),
            }
        }};
    }

    // --- Easy ---

    #[test]
    fn test_sum_of_list() {
        test_problem!(sum_of_list(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_filter_even() {
        test_problem!(filter_even(&[1, 2, 3, 4, 5, 6, 7, 8]), vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_uppercase_all() {
        test_problem!(
            uppercase_all(&["hello", "world", "rust"]),
            vec!["HELLO".to_string(), "WORLD".to_string(), "RUST".to_string()]
        );
    }

    #[test]
    fn test_count_negatives() {
        test_problem!(count_negatives(&[3, -1, 4, -1, -5, 9, 2, -6]), 4);
    }

    // --- Medium ---

    #[test]
    fn test_double_and_filter() {
        test_problem!(double_and_filter(&[1, 2, 3, 4, 5]), vec![6, 8, 10]);
    }

    #[test]
    fn test_longest_word() {
        test_problem!(
            longest_word(&["apple", "fig", "banana", "kiwi"]),
            Some("banana".to_string())
        );
    }

    #[test]
    fn test_first_word_starting_with() {
        test_problem!(
            first_word_starting_with(&["peach", "apricot", "plum", "avocado", "pear"], 'a'),
            Some("apricot".to_string())
        );
    }

    #[test]
    fn test_zip_and_sum() {
        test_problem!(
            zip_and_sum(&[1, 2, 3, 4], &[10, 20, 30, 40]),
            vec![11, 22, 33, 44]
        );
    }

    // --- Hard ---

    #[test]
    fn test_count_words_longer_than_average() {
        // Average length = 35 / 6 ≈ 5.83
        // Words above average: "elephant"(8), "butterfly"(9), "rhinoceros"(10) → 3
        test_problem!(
            count_words_longer_than_average(&[
                "cat", "elephant", "ox", "butterfly", "ant", "rhinoceros"
            ]),
            3
        );
    }

    #[test]
    fn test_second_largest() {
        test_problem!(second_largest(&[4, 1, 7, 3, 9, 2, 8]), 8);
    }
}
