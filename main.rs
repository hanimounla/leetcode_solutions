pub mod solutions;
use std::time::Instant;

use crate::solutions::{
    MinWindowSubstring::MinWindowSubstring,
    length_of_longest_substring::length_of_longest_substring,
    longest_palindrome::longest_palindrome,
    longest_palindrome_optimized::longest_palindrome_optimized, string_to_integer::my_atoi,
    zigzag_conversion::convert,
};

pub fn main() {
    //3. length of longest substring
    let test_case3 = "abcabcbb".to_string();
    println!("Running length of longest substring...");
    let start = Instant::now();
    let result3 = length_of_longest_substring(test_case3);
    let duration = start.elapsed();
    println!("{}", result3);
    println!("Elapsed: {:?}", duration);
    // 5. longest palindrom
    let test_case5 = "mwwfjysbkebpdjyabcfkgprtxpwvhglddhmvaprcvrnuxifcrjpdgnktvmggmguiiquibmtviwjsqwtchkqgxqwljouunurcdtoeygdqmijdympcamawnlzsxucbpqtuwkjfqnzvvvigifyvymfhtppqamlgjozvebygkxawcbwtouaankxsjrteeijpuzbsfsjwxejtfrancoekxgfyangvzjkdskhssdjvkvdskjtiybqgsmpxmghvvicmjxqtxdowkjhmlnfcpbtwvtmjhnzntxyfxyinmqzivxkwigkondghzmbioelmepgfttczskvqfejfiibxjcuyevvpawybcvvxtxycrfbcnpvkzryrqujqaqhoagdmofgdcbhvlwgwmsmhomknbanvntspvvhvccedzzngdywuccxrnzbtchisdwsrfdqpcwknwqvalczznilujdrlevncdsyuhnpmheukottewtkuzhookcsvctsqwwdvfjxifpfsqxpmpwospndozcdbfhselfdltmpujlnhfzjcgnbgprvopxklmlgrlbldzpnkhvhkybpgtzipzotrgzkdrqntnuaqyaplcybqyvidwcfcuxinchretgvfaepmgilbrtxgqoddzyjmmupkjqcypdpfhpkhitfegickfszermqhkwmffdizeoprmnlzbjcwfnqyvmhtdekmfhqwaftlyydirjnojbrieutjhymfpflsfemkqsoewbojwluqdckmzixwxufrdpqnwvwpbavosnvjqxqbosctttxvsbmqpnolfmapywtpfaotzmyjwnd".to_string();
    println!("Running longest palindrom...");
    let start = Instant::now();
    let result5 = longest_palindrome(test_case5.clone());
    let duration = start.elapsed();
    println!("{}", result5);
    println!("Elapsed: {:?}", duration);

    // longest palindrom optimized
    println!("Running longest palindrom optimized...");
    let start = Instant::now();
    let result5 = longest_palindrome_optimized(test_case5);
    let duration = start.elapsed();
    println!("{}", result5);
    println!("Elapsed: {:?}", duration);

    // 6. Zigzag conversion
    let test_case6 = "PAYPALISHIRING".to_string();
    println!("Running Zigzag conversion...");
    let start = Instant::now();
    let result6 = convert(test_case6, 4);
    let duration = start.elapsed();
    println!("{}", result6);
    println!("Elapsed: {:?}", duration);

    // 8. String to Integer (atoi)
    let test_case7 = "1234567890123456789012345678901234567890".to_string();
    println!("Running String to Integrer...");
    let start = Instant::now();
    let result7 = my_atoi(test_case7);
    let duration = start.elapsed();
    println!("{}", result7);
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
