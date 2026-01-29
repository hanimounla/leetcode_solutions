pub fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    let (mut max_len, mut start) = (0, 0);
    let s_chars: Vec<char> = s.chars().collect();

    for (i, &ch) in s_chars.iter().enumerate() {
        if let Some(&prev_index) = map.get(&ch) {
            if prev_index >= start {
                start = prev_index + 1;
            }
        }
        map.insert(ch, i);
        max_len = max_len.max(i - start + 1);
    }

    max_len as i32
}
