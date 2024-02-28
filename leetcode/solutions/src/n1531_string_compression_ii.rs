/**
 * [1531] String Compression II
 *
 * <a href="http://en.wikipedia.org/wiki/Run-length_encoding">Run-length encoding</a> is a string compression method that works by replacing consecutive identical characters (repeated 2 or more times) with the concatenation of the character and the number marking the count of the characters (length of the run). For example, to compress the string "aabccc" we replace <font face="monospace">"aa"</font> by <font face="monospace">"a2"</font> and replace <font face="monospace">"ccc"</font> by <font face="monospace">"c3"</font>. Thus the compressed string becomes <font face="monospace">"a2bc3".</font>
 * Notice that in this problem, we are not adding '1' after single characters.
 * Given a string s and an integer k. You need to delete at most k characters from s such that the run-length encoded version of s has minimum length.
 * Find the minimum length of the run-length encoded version of s after deleting at most k characters.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "aaabcccd", k = 2
 * Output: 4
 * Explanation: Compressing s without deleting anything will give us "a3bc3d" of length 6. Deleting any of the characters 'a' or 'c' would at most decrease the length of the compressed string to 5, for instance delete 2 'a' then we will have s = "abcccd" which compressed is abc3d. Therefore, the optimal way is to delete 'b' and 'd', then the compressed version of s will be "a3c3" of length 4.
 * <strong class="example">Example 2:
 *
 * Input: s = "aabbaa", k = 2
 * Output: 2
 * Explanation: If we delete both 'b' characters, the resulting compressed string would be "a4" of length 2.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "aaaaaaaaaaa", k = 0
 * Output: 3
 * Explanation: Since k is zero, we cannot delete anything. The compressed string is "a11" of length 3.
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 100
 *     0 <= k <= s.length
 *     s contains only lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::*;
impl Solution {
	pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
		Self::solve(
			s.as_bytes(),
			(0, 0),
			k,
			&mut vec![HashMap::new(); s.len() + 1],
		)
	}

	// FIXME: caching too brutal
	fn solve(
		s: &[u8],
		prv: (u8, i32),
		k: i32,
		cache: &mut [HashMap<((u8, i32), i32), i32>],
	) -> i32 {
		if let Some(&ans) = cache[s.len()].get(&(prv, k)) {
			return ans;
		}
		match s {
			[] => Self::encode_len(prv.1),
			[c, rest @ ..] => {
				let mut ans = Self::solve(rest, (*c, 1), k, cache) + Self::encode_len(prv.1);
				if *c == prv.0 {
					ans = ans.min(Self::solve(rest, (*c, prv.1 + 1), k, cache));
				}
				if k > 0 {
					ans = ans.min(Self::solve(rest, prv, k - 1, cache));
				}
				cache[s.len()].insert((prv, k), ans);
				ans
			}
		}
	}

	#[inline]
	fn encode_len(x: i32) -> i32 {
		match x {
			2.. => x.to_string().len() as i32 + 1,
			x => x,
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1531() {
		assert_eq!(
			Solution::get_length_of_optimal_compression("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(), 97),
			2
		);
		assert_eq!(
			Solution::get_length_of_optimal_compression("aabaabbcbbbaccc".to_owned(), 6),
			4
		);
		assert_eq!(
			Solution::get_length_of_optimal_compression("aaaaaaaaaaa".to_owned(), 0),
			3
		);
		assert_eq!(
			Solution::get_length_of_optimal_compression("abc".to_owned(), 2),
			1
		);
		assert_eq!(
			Solution::get_length_of_optimal_compression("aaabcccd".to_owned(), 2),
			4
		);
		assert_eq!(
			Solution::get_length_of_optimal_compression("aabbaa".to_owned(), 2),
			2
		);
	}
}
