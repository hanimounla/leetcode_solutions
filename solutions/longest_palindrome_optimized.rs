pub fn longest_palindrome_optimized(input: String) -> String {
    if input.is_empty() {
        return String::new();
    }
    let chars: Vec<char> = input.chars().collect();
    let mut start = 0;
    let mut max_len = 1;

    // Check for palindromes centered at each position
    for i in 0..chars.len() {
        // Check odd-length palindromes (center at i)
        let len1 = expand_around_center(&chars, i, i);
        // Check even-length palindromes (center between i and i+1)
        let len2 = expand_around_center(&chars, i, i + 1);

        let len = len1.max(len2);
        if len > max_len {
            max_len = len;
            start = i - (len - 1) / 2;
        }
    }

    chars[start..start + max_len].iter().collect()
}

fn expand_around_center(chars: &[char], mut left: usize, mut right: usize) -> usize {
    while right < chars.len() && chars[left] == chars[right] {
        if left == 0 {
            return right - left + 1;
        }
        left -= 1;
        right += 1;
    }

    // Return length of palindrome (we expanded one step too far)
    right - left - 1
}
