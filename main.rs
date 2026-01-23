pub mod solutions;
use std::time::Instant;

use crate::solutions::{
    longest_palindrome::longest_palindrome,
    longest_palindrome_optimized::longest_palindrome_optimized, zigzag_conversion::convert,
};

pub fn main() {
    // 5.
    // longest palindrom
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

    // 6.
    // Zigzag conversion
    let test_case6 = "PAYPALISHIRING".to_string();
    println!("Running Zigzag conversion...");
    let start = Instant::now();
    let result6 = convert(test_case6, 4);
    let duration = start.elapsed();
    println!("{}", result6);
    println!("Elapsed: {:?}", duration);
}
