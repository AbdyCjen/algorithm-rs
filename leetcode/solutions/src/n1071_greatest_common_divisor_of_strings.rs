/**
 * [1071] Greatest Common Divisor of Strings
 *
 * For two strings s and t, we say "t divides s" if and only if s = t + ... + t (i.e., t is concatenated with itself one or more times).
 * Given two strings str1 and str2, return the largest string x such that x divides both str1 and str2.
 *  
 * <strong class="example">Example 1:
 *
 * Input: str1 = "ABCABC", str2 = "ABC"
 * Output: "ABC"
 *
 * <strong class="example">Example 2:
 *
 * Input: str1 = "ABABAB", str2 = "ABAB"
 * Output: "AB"
 *
 * <strong class="example">Example 3:
 *
 * Input: str1 = "LEET", str2 = "CODE"
 * Output: ""
 *
 *  
 * Constraints:
 *
 *     1 <= str1.length, str2.length <= 1000
 *     str1 and str2 consist of English uppercase letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn gcd_of_strings(str1: String, str2: String) -> String {
		let lcs = str1
			.bytes()
			.zip(str2.bytes())
			.take_while(|(c1, c2)| c1 == c2)
			.count();
		for i in (1..=lcs).rev() {
			if str1.len() % i == 0
				&& str2.len() % i == 0
				&& str1[..i]
					.bytes()
					.cycle()
					.zip(str1.bytes())
					.all(|(c1, c2)| c1 == c2)
				&& str1[..i]
					.bytes()
					.cycle()
					.zip(str2.bytes())
					.all(|(c1, c2)| c1 == c2)
			{
				return str1[..i].to_owned();
			}
		}
		"".to_owned()
	}
}

// submission codes end
