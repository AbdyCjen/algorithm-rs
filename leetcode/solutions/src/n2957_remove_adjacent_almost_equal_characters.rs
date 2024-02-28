/**
 * [2957] Remove Adjacent Almost-Equal Characters
 *
 * You are given a 0-indexed string word.
 * In one operation, you can pick any index i of word and change word[i] to any lowercase English letter.
 * Return the minimum number of operations needed to remove all adjacent almost-equal characters from word.
 * Two characters a and b are almost-equal if a == b or a and b are adjacent in the alphabet.
 *  
 * <strong class="example">Example 1:
 *
 * Input: word = "aaaaa"
 * Output: 2
 * Explanation: We can change word into "a<u>c</u>a<u>c</u>a" which does not have any adjacent almost-equal characters.
 * It can be shown that the minimum number of operations needed to remove all adjacent almost-equal characters from word is 2.
 *
 * <strong class="example">Example 2:
 *
 * Input: word = "abddez"
 * Output: 2
 * Explanation: We can change word into "<u>y</u>bd<u>o</u>ez" which does not have any adjacent almost-equal characters.
 * It can be shown that the minimum number of operations needed to remove all adjacent almost-equal characters from word is 2.
 * <strong class="example">Example 3:
 *
 * Input: word = "zyxyxyz"
 * Output: 3
 * Explanation: We can change word into "z<u>a</u>x<u>a</u>x<u>a</u>z" which does not have any adjacent almost-equal characters.
 * It can be shown that the minimum number of operations needed to remove all adjacent almost-equal characters from word is 3.
 *
 *  
 * Constraints:
 *
 *     1 <= word.length <= 100
 *     word consists only of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn remove_almost_equal_characters(word: String) -> i32 {
		let mut i = 1;
		let word = word.into_bytes();
		let mut ans = 0;
		while i < word.len() {
			if (word[i] as i32 - word[i - 1] as i32).abs() <= 1 {
				ans += 1;
				i += 1;
			}
			i += 1;
		}
		ans
	}
}

// submission codes end
