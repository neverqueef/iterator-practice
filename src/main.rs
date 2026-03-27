#![allow(dead_code, unused_variables)]

use std::fmt::Debug;

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
    numbers.iter().sum()
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
    numbers.iter().filter(|&&n| n % 2==0).copied().collect()
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
/// Hints: Start with (i32::MIN, i32::MIN) and update both values as you walk through the iterator.
/// Statements needed: 1
fn second_largest(numbers: &[i32]) -> i32 {
    todo!()
}

// ============================================================================= 
// Main and Test Harness
// =============================================================================
fn main() {
    let mut results: Vec<Option<bool>> = Vec::new();

    println!("── Easy ──────────────────────────────────────────────────");
    results.push(check("sum_of_list", "[1, 2, 3, 4, 5]",
        || sum_of_list(&[1, 2, 3, 4, 5]), 15));
    results.push(check("filter_even", "[1, 2, 3, 4, 5, 6, 7, 8]",
        || filter_even(&[1, 2, 3, 4, 5, 6, 7, 8]), vec![2, 4, 6, 8]));
    results.push(check(r#"uppercase_all(["hello", "world", "rust"])"#, r#"["hello", "world", "rust"]"#,
        || uppercase_all(&["hello", "world", "rust"]),
        vec!["HELLO".to_string(), "WORLD".to_string(), "RUST".to_string()]));
    results.push(check("count_negatives", "[3, -1, 4, -1, -5, 9, 2, -6]",
        || count_negatives(&[3, -1, 4, -1, -5, 9, 2, -6]), 4));

    println!("\n── Medium ────────────────────────────────────────────────");
    results.push(check("double_and_filter", "[1, 2, 3, 4, 5]",
        || double_and_filter(&[1, 2, 3, 4, 5]), vec![6, 8, 10]));
    results.push(check("longest_word", r#"["apple", "fig", "banana", "kiwi"]"#,
        || longest_word(&["apple", "fig", "banana", "kiwi"]),
        Some("banana".to_string())));
    results.push(check("first_word_starting_with", r#"["peach", "apricot", "plum", "avocado", "pear"], 'a'"#,
        || first_word_starting_with(&["peach", "apricot", "plum", "avocado", "pear"], 'a'),
        Some("apricot".to_string())));
    results.push(check("zip_and_sum", "a = [1, 2, 3, 4], b = [10, 20, 30, 40]",
        || zip_and_sum(&[1, 2, 3, 4], &[10, 20, 30, 40]), vec![11, 22, 33, 44]));

    println!("\n── Hard ──────────────────────────────────────────────────");
    results.push(check("count_words_longer_than_average",
        r#"["cat", "elephant", "ox", "butterfly", "ant", "rhinoceros"]"#,
        || count_words_longer_than_average(&["cat", "elephant", "ox", "butterfly", "ant", "rhinoceros"]),
        3));
    results.push(check("second_largest", "[4, 1, 7, 3, 9, 2, 8]",
        || second_largest(&[4, 1, 7, 3, 9, 2, 8]), 8));

    let implemented = results.iter().filter(|r| r.is_some()).count();
    let correct = results.iter().filter(|r| **r == Some(true)).count();

    println!("\n── Results ───────────────────────────────────────────────");
    if implemented == 0 {
        println!("No problems implemented yet. Replace a todo!() with your solution!");
    } else {
        println!("{implemented}/10 implemented  ·  {correct}/{implemented} correct");
    }
}

/// Runs `f` and compares its output to `expected`, printing the result.
/// Returns Some(true) if correct, Some(false) if wrong, None if not yet implemented.
fn check<T>(name: &str, input: &str, f: impl FnOnce() -> T, expected: T) -> Option<bool>
where
    T: Debug + PartialEq,
{
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {})); // silence the default "panicked at" message
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    std::panic::set_hook(prev_hook);

    match result {
        Err(e) => {
            let is_todo = e
                .downcast_ref::<&str>()
                .map(|s| s.contains("not yet implemented"))
                .unwrap_or(false);
            if !is_todo {
                std::panic::resume_unwind(e);
            }
            None
        }
        Ok(actual) => {
            let passed = actual == expected;
            let mark = if passed { "✓" } else { "✗" };
            println!("\n  {mark} {name}");
            println!("    input:    {input}");
            println!("    output:   {:?}", actual);
            if !passed {
                println!("    expected: {:?}", expected);
            }
            Some(passed)
        }
    }
}

// ───────────────────────────────────────────────────────────────