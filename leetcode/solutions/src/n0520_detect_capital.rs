/**
 * [520] Detect Capital
 *
 * We define the usage of capitals in a word to be right when one of the following cases holds:
 *
 *     All letters in this word are capitals, like "USA".
 *     All letters in this word are not capitals, like "leetcode".
 *     Only the first letter in this word is capital, like "Google".
 *
 * Given a string word, return true if the usage of capitals in it is right.
 *  
 * <strong class="example">Example 1:
 * Input: word = "USA"
 * Output: true
 * <strong class="example">Example 2:
 * Input: word = "FlaG"
 * Output: false
 *  
 * Constraints:
 *
 *     1 <= word.length <= 100
 *     word consists of lowercase and uppercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn detect_capital_use(word: String) -> bool {
		let word = word.into_bytes();
		word[1..].iter().all(u8::is_ascii_lowercase) || word.iter().all(u8::is_ascii_uppercase)
	}
}

// submission codes end

#[cfg(test)]
mod tests {

	#[test]
	fn test_520() {}
}
