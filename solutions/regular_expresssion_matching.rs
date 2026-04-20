pub fn is_match(s: String, p: String) -> bool {
    use std::vec;

    // dynamic programming approach
    let mut array: vec::Vec<Vec<bool>> = vec![vec![false; p.len() + 1]; s.len() + 1];
    for i in 0..=s.len() {
        for j in 0..=p.len() {
            if j == 0 {
                array[i][j] = i == 0;
            } else if p.chars().nth(j - 1).unwrap() == '*' {
                array[i][j] = array[i][j - 2]
                    || (i > 0
                        && (s.chars().nth(i - 1).unwrap() == p.chars().nth(j - 2).unwrap()
                            || p.chars().nth(j - 2).unwrap() == '.')
                        && array[i - 1][j]);
            } else {
                array[i][j] = i > 0
                    && (s.chars().nth(i - 1).unwrap() == p.chars().nth(j - 1).unwrap()
                        || p.chars().nth(j - 1).unwrap() == '.')
                    && array[i - 1][j - 1];
            }
        }
    }

    return array[s.len()][p.len()];
    // simple case: if the string and pattern are exactly the same
    // if s == p {
    //     return true;
    // }
    // if !p.contains('.') && !p.contains('*') && s.len() != p.len() {
    //     return false;
    // }
    // let s_chars = s.chars(); // string characters
    // let p_chars = p.chars(); // pattern characters

    // let mut index = 0;

    // for (s_char, p_char) in s_chars.zip(p_chars) {
    //     if p_char == '*' && index > 0 {
    //         let prev_p_char = p.chars().nth(index - 1).unwrap();
    //         if prev_p_char == s_char || prev_p_char == '.' {
    //             continue;
    //         } else {
    //             return false;
    //         }
    //     }
    //     index += 1;
    // }

    // true
}
