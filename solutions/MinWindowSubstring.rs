#![allow(non_snake_case)]

/*
Min Window Substring

Have the function MinWindowSubstring(strArr) take the array of strings stored in strArr,
which will contain only two strings, the first parameter being the string N and the second
parameter being a string K of some characters, and your goal is to determine the smallest
substring of N that contains all the characters in K. For example: if strArr is ["aaabaaddae", "aed"]
then the smallest substring of N that contains the characters a, e, and d is "dae" located at the end of
the string. So for this example your program should return the string dae.

Another example: if strArr is ["aabdccdbcacd", "aad"] then the smallest substring of N that contains all
of the characters in K is "aabd" which is located at the beginning of the string. Both parameters will be
strings ranging in length from 1 to 50 characters and all of K's characters will exist somewhere in the string N. Both
strings will only contains lowercase alphabetic characters.

Examples
Input: vec!["ahffaksfajeeubsne", "jefaa"]
Output: aksfaje
Input: vec!["aaffhkksemckelloe", "fhea"]
Output: affhkkse

*/

// MinWindowSubstring takes a slice of 2 string slices for ergonomic usage.
pub fn MinWindowSubstring<T: std::fmt::Debug + 'static>(value: T) -> String {
    use std::any::TypeId;
    use std::collections::HashMap;
    println!("{:?}", value); // for demonstration that we have a Debug bound

    // The function's logic as written expected a &[&str] named `strArr`
    // To preserve original functionality, we can panic or return empty string if it's not that.
    // For demonstration, let's try to downcast to &[&str] (in real Rust, we'd use generic traits or enums, but per instruction follow this pattern).

    // Since T can be anything Debug, let's try to mimic previous behavior if T is &[&str]
    // We'll use std::any::TypeId for a runtime type check.

    // If T is actually &[&str], process as before
    // But in generic case, just return empty
    // (In production, you'd probably redesign instead.)
    let is_strarr = TypeId::of::<T>() == TypeId::of::<&[&str]>()
        || TypeId::of::<T>() == TypeId::of::<Vec<&str>>();

    if !is_strarr {
        return String::new();
    }

    // SAFETY: We've just checked type, so we can transmute. THIS IS NOT ROBUST, only done per the prompt's request.
    // In practice, don't use transmute this way.
    let strArr: &[&str] = if TypeId::of::<T>() == TypeId::of::<&[&str]>() {
        unsafe { std::mem::transmute_copy(&value) }
    } else {
        // If Vec<&str>
        let v: &Vec<&str> = unsafe { std::mem::transmute_copy(&value) };
        &v[..]
    };

    if strArr.len() != 2 {
        return String::new();
    }
    let n = strArr[0];
    let k = strArr[1];

    // Frequency of required chars
    let mut required_counts = HashMap::new();
    for ch in k.chars() {
        *required_counts.entry(ch).or_insert(0) += 1;
    }

    let mut left = 0;
    let mut min_window = (0, usize::MAX);
    let mut formed = 0;
    let required = required_counts.len();
    let mut window_counts: HashMap<char, usize> = HashMap::new();

    let n_chars: Vec<char> = n.chars().collect();

    for right in 0..n_chars.len() {
        let ch = n_chars[right];
        *window_counts.entry(ch).or_insert(0) += 1;

        if required_counts.contains_key(&ch) && window_counts[&ch] == required_counts[&ch] {
            formed += 1;
        }

        // Shrink window from left as long as it's 'desirable'
        while left <= right && formed == required {
            // Update min window
            if right - left < min_window.1 - min_window.0 {
                min_window = (left, right);
            }

            let lch = n_chars[left];
            *window_counts.get_mut(&lch).unwrap() -= 1;

            if required_counts.contains_key(&lch) && window_counts[&lch] < required_counts[&lch] {
                formed -= 1;
            }

            left += 1;
        }
    }

    if min_window.1 == usize::MAX {
        String::new()
    } else {
        n_chars[min_window.0..=min_window.1].iter().collect()
    }
}
