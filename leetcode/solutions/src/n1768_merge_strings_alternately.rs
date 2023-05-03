/**
 * [1768] Merge Strings Alternately
 *
 * You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.
 *
 * Return the merged string.
 *
 *  
 * <strong class="example">Example 1:
 *
 *
 * Input: word1 = "abc", word2 = "pqr"
 * Output: "apbqcr"
 * Explanation: The merged string will be merged as so:
 * word1:  a   b   c
 * word2:    p   q   r
 * merged: a p b q c r
 *
 *
 * <strong class="example">Example 2:
 *
 *
 * Input: word1 = "ab", word2 = "pqrs"
 * Output: "apbqrs"
 * Explanation: Notice that as word2 is longer, "rs" is appended to the end.
 * word1:  a   b
 * word2:    p   q   r   s
 * merged: a p b q   r   s
 *
 *
 * <strong class="example">Example 3:
 *
 *
 * Input: word1 = "abcd", word2 = "pq"
 * Output: "apbqcd"
 * Explanation: Notice that as word1 is longer, "cd" is appended to the end.
 * word1:  a   b   c   d
 * word2:    p   q
 * merged: a p b q c   d
 *
 *
 *  
 * Constraints:
 *
 *
 *     1 <= word1.length, word2.length <= 100
 *     word1 and word2 consist of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn merge_alternately(word1: String, word2: String) -> String {
		let mut ans = Vec::with_capacity(word1.len() + word2.len());
		for (c1, c2) in word1.bytes().zip(word2.bytes()) {
			ans.push(c1);
			ans.push(c2);
		}
		let s = word1.len().min(word2.len());
		if word1.len() > s {
			ans.extend(word1.bytes().skip(s));
		} else {
			ans.extend(word2.bytes().skip(s));
		}
		String::from_utf8(ans).unwrap()
	}
}

// submission codes end
