pub mod solutions;
use std::time::Instant;

use crate::solutions::{
    MinWindowSubstring::MinWindowSubstring, container_with_most_water::max_area,
    length_of_longest_substring::length_of_longest_substring,
    longest_palindrome::longest_palindrome,
    longest_palindrome_optimized::longest_palindrome_optimized, palindrom_number::is_palindrome,
    regular_expresssion_matching::is_match, string_to_integer::my_atoi, zigzag_conversion::convert,
};

pub fn main() {
    //3. length of longest substring
    let test_case3 = "abcabcbb".to_string();
    println!("3. length of longest substring...");
    let start = Instant::now();
    let result3 = length_of_longest_substring(test_case3);
    let duration = start.elapsed();
    println!("{}", result3);
    println!("Elapsed: {:?}", duration);
    // 5. longest palindrom
    let test_case5 = "mwwfjysbkebpdjyabcfkgprtxpwvhglddhmvaprcvrnuxifcrjpdgnktvmggmguiiquibmtviwjsqwtchkqgxqwljouunurcdtoeygdqmijdympcamawnlzsxucbpqtuwkjfqnzvvvigifyvymfhtppqamlgjozvebygkxawcbwtouaankxsjrteeijpuzbsfsjwxejtfrancoekxgfyangvzjkdskhssdjvkvdskjtiybqgsmpxmghvvicmjxqtxdowkjhmlnfcpbtwvtmjhnzntxyfxyinmqzivxkwigkondghzmbioelmepgfttczskvqfejfiibxjcuyevvpawybcvvxtxycrfbcnpvkzryrqujqaqhoagdmofgdcbhvlwgwmsmhomknbanvntspvvhvccedzzngdywuccxrnzbtchisdwsrfdqpcwknwqvalczznilujdrlevncdsyuhnpmheukottewtkuzhookcsvctsqwwdvfjxifpfsqxpmpwospndozcdbfhselfdltmpujlnhfzjcgnbgprvopxklmlgrlbldzpnkhvhkybpgtzipzotrgzkdrqntnuaqyaplcybqyvidwcfcuxinchretgvfaepmgilbrtxgqoddzyjmmupkjqcypdpfhpkhitfegickfszermqhkwmffdizeoprmnlzbjcwfnqyvmhtdekmfhqwaftlyydirjnojbrieutjhymfpflsfemkqsoewbojwluqdckmzixwxufrdpqnwvwpbavosnvjqxqbosctttxvsbmqpnolfmapywtpfaotzmyjwnd".to_string();
    println!("5. Longest palindrom...");
    let start = Instant::now();
    let result5 = longest_palindrome(test_case5.clone());
    let duration = start.elapsed();
    println!("{}", result5);
    println!("Elapsed: {:?}", duration);

    // longest palindrom optimized
    println!("5. Longest palindrom optimized...");
    let start = Instant::now();
    let result5 = longest_palindrome_optimized(test_case5);
    let duration = start.elapsed();
    println!("{}", result5);
    println!("Elapsed: {:?}", duration);

    // 6. Zigzag conversion
    let test_case6 = "PAYPALISHIRING".to_string();
    println!("6. Zigzag conversion...");
    let start = Instant::now();
    let result6 = convert(test_case6, 4);
    let duration = start.elapsed();
    println!("{}", result6);
    println!("Elapsed: {:?}", duration);

    // 8.
    // String to integer
    let test_case8 = "words and 987".to_string();
    println!("8. String to integer...");
    let start = Instant::now();
    let result8 = my_atoi(test_case8);
    let duration = start.elapsed();
    println!("{}", result8);
    println!("Elapsed: {:?}", duration);

    // 9. Palindrome number
    let test_case9 = -121;
    println!("9. Palindrome number...");
    let start = Instant::now();
    let result9 = is_palindrome(test_case9);
    let duration = start.elapsed();
    println!("{}", result9);
    println!("Elapsed: {:?}", duration);

    // 10. Regular expression matching
    println!("10. Regular expression matching...");
    let start = Instant::now();
    let result10 = is_match("mississippi".to_string(), "mis*is*ip*.".to_string());
    let duration = start.elapsed();
    println!("{}", result10);
    println!("Elapsed: {:?}", duration);

    // 11. Container with most water
    let test_case11 = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    println!("11. Container with most water...");
    let start = Instant::now();
    let result11 = max_area(test_case11);
    let duration = start.elapsed();
    println!("{}", result11);
    println!("Elapsed: {:?}", duration);

    // Extra solutions

    // MinWindowSubstring
    let test_list = vec!["ahffaksfajeeubsne", "jefaa"];
    println!("Running MinWindowSubstring...");
    let start = Instant::now();
    let result = MinWindowSubstring(test_list);
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
}
