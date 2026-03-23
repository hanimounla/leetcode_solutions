pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}
