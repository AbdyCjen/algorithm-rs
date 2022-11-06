/**
 * [2131] Longest Palindrome by Concatenating Two Letter Words
 *
 * You are given an array of strings words. Each element of words consists of two lowercase English letters.
 * Create the longest possible palindrome by selecting some elements from words and concatenating them in any order. Each element can be selected at most once.
 * Return the length of the longest palindrome that you can create. If it is impossible to create any palindrome, return 0.
 * A palindrome is a string that reads the same forward and backward.
 *  
 * <strong class="example">Example 1:
 *
 * Input: words = ["lc","cl","gg"]
 * Output: 6
 * Explanation: One longest palindrome is "lc" + "gg" + "cl" = "lcggcl", of length 6.
 * Note that "clgglc" is another longest palindrome that can be created.
 *
 * <strong class="example">Example 2:
 *
 * Input: words = ["ab","ty","yt","lc","cl","ab"]
 * Output: 8
 * Explanation: One longest palindrome is "ty" + "lc" + "cl" + "yt" = "tylcclyt", of length 8.
 * Note that "lcyttycl" is another longest palindrome that can be created.
 *
 * <strong class="example">Example 3:
 *
 * Input: words = ["cc","ll","xx"]
 * Output: 2
 * Explanation: One longest palindrome is "cc", of length 2.
 * Note that "ll" is another longest palindrome that can be created, and so is "xx".
 *
 *  
 * Constraints:
 *
 *     1 <= words.length <= 10^5
 *     words[i].length == 2
 *     words[i] consists of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn longest_palindrome(words: Vec<String>) -> i32 {
		let mut cnts = [[0; 26]; 26];
		let mut ans = 0;
		for word in words.into_iter().map(String::into_bytes) {
			let (first, second) = ((word[0] - b'a') as usize, (word[1] - b'a') as usize);
			if cnts[second][first] > 0 {
				ans += 4;
				cnts[second][first] -= 1;
			} else {
				cnts[first][second] += 1;
			}
		}
		for (i, row) in cnts.iter().enumerate() {
			if row[i] > 0 {
				return ans + 2;
			}
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2131() {
		assert_eq!(
			Solution::longest_palindrome(vec_string!["lc", "cl", "gg"]),
			6
		);
		assert_eq!(
			Solution::longest_palindrome(vec_string!["lc", "cl", "gg", "gg", "gg"]),
			10
		);
		assert_eq!(
			Solution::longest_palindrome(vec_string!["ab", "ty", "yt", "lc", "cl", "ab"]),
			8
		);
		assert_eq!(
			Solution::longest_palindrome(vec_string!["cc", "ll", "xx"]),
			2
		);
	}
}
