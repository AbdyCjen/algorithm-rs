/**
 * [451] Sort Characters By Frequency
 *
 * Given a string s, sort it in decreasing order based on the frequency of the characters. The frequency of a character is the number of times it appears in the string.
 * Return the sorted string. If there are multiple answers, return any of them.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "tree"
 * Output: "eert"
 * Explanation: 'e' appears twice while 'r' and 't' both appear once.
 * So 'e' must appear before both 'r' and 't'. Therefore "eetr" is also a valid answer.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "cccaaa"
 * Output: "aaaccc"
 * Explanation: Both 'c' and 'a' appear three times, so both "cccaaa" and "aaaccc" are valid answers.
 * Note that "cacaca" is incorrect, as the same characters must be together.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "Aabb"
 * Output: "bbAa"
 * Explanation: "bbaA" is also a valid answer, but "Aabb" is incorrect.
 * Note that 'A' and 'a' are treated as two different characters.
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 5 * 10^5
 *     s consists of uppercase and lowercase English letters and digits.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn frequency_sort(s: String) -> String {
		let mut cnt = std::collections::HashMap::new();
		for c in s.into_bytes() {
			*cnt.entry(c).or_insert(0) += 1;
		}
		let mut cnt = cnt.into_iter().collect::<Vec<_>>();
		cnt.sort_unstable_by_key(|i| i.1);

		let mut ans = Vec::new();
		for (c, cnt) in cnt.into_iter().rev() {
			for _ in 0..cnt {
				ans.push(c);
			}
		}
		String::from_utf8(ans).unwrap()
	}
}

// submission codes end
