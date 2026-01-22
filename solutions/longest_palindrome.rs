pub fn longest_palindrome(input: String) -> String {
    let mut result: String = Default::default();
    let mut temp: String = Default::default();
    let mut index = 0;
    for _ in input.chars() {
        let mut input_mut = String::from(&input);
        let after_char_input = input_mut.split_off(index);
        for c2 in after_char_input.chars() {
            temp += &c2.to_string();
            if check_palindrom(&temp) && temp.len() > result.len() {
                result = temp.clone();
            }
        }
        index += 1;
        temp = Default::default();
    }
    return result;

    fn check_palindrom(input: &str) -> bool {
        let reveresed = input.chars().rev().collect::<String>();
        return input == reveresed;
    }
}
