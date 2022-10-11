//! Assignment 3: Mastering common programming concepts (2/2).
//!
//! The primary goal of this assignment is to re-learn the common programming concepts in Rust, especially those in the Rust Book chapters 6, 7, 8, and 9.
//! Please make sure you're comfortable with the concepts to proceed on to the next assignments.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-03.sh` works fine.
//! See `assignment03_grade.rs` and `/scripts/grade-03.sh` for the test script.

use std::collections::{HashMap, HashSet};

/// Day of week.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayOfWeek {
    /// Sunday.
    Sun,
    /// Monday.
    Mon,
    /// Tuesday.
    Tue,
    /// Wednesday.
    Wed,
    /// Thursday.
    Thu,
    /// Friday.
    Fri,
    /// Saturday.
    Sat,
}

/// The next day of week.
///
/// `next_weekday(Thu)` is `Fri`; and `next_weekday(Fri)` is `Mon`.
pub fn next_weekday(day: DayOfWeek) -> DayOfWeek {
    match day {
        DayOfWeek::Mon => DayOfWeek::Tue,
        DayOfWeek::Tue => DayOfWeek::Wed,
        DayOfWeek::Wed => DayOfWeek::Thu,
        DayOfWeek::Thu => DayOfWeek::Fri,
        DayOfWeek::Fri | DayOfWeek::Sat | DayOfWeek::Sun => DayOfWeek::Mon,
    }
}

/// Custom option type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MyOption<T> {
    /// Some value of type `T`.
    MySome(T),
    /// No value.
    MyNone,
}

/// Maps an `MyOption<T>` to `MyOption<U>` by applying a function to a contained value.
///
/// # Examples
///
/// Converts an `MyOption<String>` into an `MyOption<usize>`, consuming the original:
///
/// ```
/// fn len(s: String) -> usize {
///     s.len()
/// }
///
/// assert_eq!(my_map(MyOption::MySome(String::from("Hello, World!")), len), MyOption::MySome(13));
/// assert_eq!(my_map(MyOption::MyNone, len), MyOption::MyNone);
/// ```
pub fn my_map<T, U, F: FnOnce(T) -> U>(v: MyOption<T>, f: F) -> MyOption<U> {
    match v {
        MyOption::MySome(value) => MyOption::MySome(f(value)),
        MyOption::MyNone => MyOption::MyNone,
    }
}

/// Returns `MyNone` if the option is `MyNone`, otherwise calls `f` with the wrapped value and returns the result.
///
/// Some languages call this operation flatmap.
///
/// # Examples
///
/// ```
/// fn pos_then_to_string(x: isize) -> MyOption<String> {
///     if x > 0 {
///         MyOption::MySome(x.to_string())
///     } else {
///         MyOption::MyNone
///     }
/// }
///
/// assert_eq!(my_and_then(MyOption::MySome(2), pos_then_to_string), MyOption::MySome(2.to_string()));
/// assert_eq!(my_and_then(MyOption::MySome(-3), pos_then_to_string), MyOption::MyNone);
/// assert_eq!(my_and_then(MyOption::MyNone, pos_then_to_string), MyOption::MyNone);
/// ```
pub fn my_and_then<T, U, F: FnOnce(T) -> MyOption<U>>(v: MyOption<T>, f: F) -> MyOption<U> {
    match v {
        MyOption::MySome(value) => f(value),
        MyOption::MyNone => MyOption::MyNone,
    }
}

/// Given a list of integers, returns its median (when sorted, the value in the middle position).
///
/// For a data set `x` of `n` elements, the median can be defined as follows:
///
/// - If `n` is odd, the median is `(n+1)/2`-th smallest element of `x`.
/// - If `n` is even, the median is `(n/2)+1`-th smallest element of `x`.
///
/// For example, the following list of seven numbers,
///
/// ```
/// vec![1, 3, 3, 6, 7, 8, 9]
/// ```
///
/// has the median of 6, which is the fourth value. And for this data set of eight numbers,
///
/// ```
/// vec![1, 2, 3, 4, 5, 6, 8, 9]
/// ```
///
/// it has the median of 5, which is the fifth value.
///
/// Returns `None` if the list is empty.
pub fn median(values: Vec<isize>) -> Option<isize> {
    let mut value = vec![];
    for i in values {
        value.push(i)
    }
    value.sort();
    if value.is_empty() {
        return None;
    }
    if value.len() % 2 == 0 {
        Some(value[(value.len() + 1) / 2])
    } else {
        Some(value[value.len() / 2])
    }
}

/// Given a list of integers, returns its smallest mode (the value that occurs most often; a hash map will be helpful here).
///
/// Returns `None` if the list is empty.
pub fn mode(values: Vec<isize>) -> Option<isize> {
    if values.is_empty() {
        return None;
    }
    let mut num_map = HashMap::new();
    let mut max_num = 0;
    let mut max_mode = 0;
    for i in values {
        let entry = num_map.entry(i).or_insert(0);
        *entry += 1;
        if *entry > max_num {
            max_num = *entry;
            max_mode = i;
        }
    }
    Some(max_mode)
}

/// Converts the given string to Pig Latin. Use the rules below to translate normal English into Pig Latin.
///
/// 1. If a word starts with a consonant and a vowel, move the first letter of the word at the end of the word and add "ay".
///
/// Example: "happy" -> "appyh" + "ay" -> "appyhay"
///
/// 2. If a word starts with multiple consonants, move them to the end of the word and add "ay".
///
/// Example: "string" -> "ingstr" + "ay" -> "ingstray"
///
/// 3. If a word starts with a vowel, add the word "hay" at the end of the word.
///
/// Example: "explain" -> "explain" + "hay" -> "explainhay"
///
/// Keep in mind the details about UTF-8 encoding!
///
/// You may assume the string only contains lowercase alphabets, and it contains at least one vowel.
pub fn piglatin(input: String) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut vowel = 0_usize;
    for (i, &ch) in chars.iter().enumerate() {
        if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
            vowel = i;
            break;
        }
    }
    let mut s = input;
    if vowel == 0 {
        s.push_str("hay");
        s
    } else if vowel == 1 {
        let ch = s.remove(0);
        s.push(ch);
        s.push_str("ay");
        s
    } else {
        let slice_front = &s[0..vowel];
        let slice_end = &s[vowel..];
        let mut slice = slice_end.to_string();
        slice.push_str(slice_front);
        slice.push_str("ay");
        slice
    }
}

/// Converts HR commands to the organization table.
///
/// If the commands are as follows:
///
/// ```
/// vec!["Add Amir to Engineering", "Add Sally to Sales", "Remove Jeehoon from Sales", "Move Amir from Engineering to Sales"]
/// ```
///
/// The return value should be:
///
/// ```
/// ["Sales" -> ["Amir", "Sally"]]
/// ```
///
/// - The result is a map from department to the list of its employees.
/// - An empty department should not appear in the result.
/// - There are three commands: "Add <person> to <department>", "Remove <person> from <department>", and "Move <person> from <department> to <department>".
/// - If a command is not executable, then it's ignored.
/// - There is no space in the name of the person and department.
///
/// See the test function for more details.
pub fn organize(commands: Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut order_map = HashMap::new();

    for str in commands {
        let str_vec: Vec<&str> = str.split(' ').collect();
        if str_vec[0] == "Add" {
            let entry = order_map
                .entry(str_vec[3].to_string())
                .or_insert_with(HashSet::new);
            let _out = (*entry).insert(str_vec[1].to_string());
        } else if str_vec[0] == "Remove" {
            let entry = order_map
                .entry(str_vec[3].to_string())
                .or_insert_with(HashSet::new);
            let _out = (*entry).remove(&str_vec[1].to_string());
            if (*entry).is_empty() {
                let _out = order_map.remove(&str_vec[3].to_string());
            }
        } else if str_vec[0] == "Move" {
            let entry = order_map
                .entry(str_vec[3].to_string())
                .or_insert_with(HashSet::new);
            let out = (*entry).remove(&str_vec[1].to_string());
            if !out {
                continue;
            }
            if (*entry).is_empty() {
                let _out = order_map.remove(&str_vec[3].to_string());
            }
            let entry = order_map
                .entry(str_vec[5].to_string())
                .or_insert_with(HashSet::new);
            let _out = (*entry).insert(str_vec[1].to_string());
        }
    }
    order_map
}
