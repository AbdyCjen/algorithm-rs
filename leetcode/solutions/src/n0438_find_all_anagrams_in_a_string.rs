/**
 * [438] Find All Anagrams in a String
 *
 * Given two strings s and p, return an array of all the start indices of p's anagrams in s. You may return the answer in any order.
 * An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "cbaebabacd", p = "abc"
 * Output: [0,6]
 * Explanation:
 * The substring with start index = 0 is "cba", which is an anagram of "abc".
 * The substring with start index = 6 is "bac", which is an anagram of "abc".
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "abab", p = "ab"
 * Output: [0,1,2]
 * Explanation:
 * The substring with start index = 0 is "ab", which is an anagram of "ab".
 * The substring with start index = 1 is "ba", which is an anagram of "ab".
 * The substring with start index = 2 is "ab", which is an anagram of "ab".
 *
 *  
 * Constraints:
 *
 *     1 <= s.length, p.length <= 3 * 10^4
 *     s and p consist of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
		let mut cnt = [0; 26];
		for c in p.bytes() {
			cnt[(c - b'a') as usize] += 1;
		}

		let mut ans = vec![];
		let mut i = 0;
		for (j, c) in s.bytes().enumerate() {
			cnt[(c - b'a') as usize] -= 1;
			while cnt[(c - b'a') as usize] < 0 {
				cnt[(s.as_bytes()[i] - b'a') as usize] += 1;
				i += 1;
			}
			if j + 1 - i == p.len() && cnt == [0; 26] {
				ans.push(i as i32);
			}
		}
		ans
	}
}

// submission codes end
