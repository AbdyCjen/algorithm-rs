/**
 * [1312] Minimum Insertion Steps to Make a String Palindrome
 *
 * Given a string s. In one step you can insert any character at any index of the string.
 * Return the minimum number of steps to make s palindrome.
 * A Palindrome String is one that reads the same backward as well as forward.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "zzazz"
 * Output: 0
 * Explanation: The string "zzazz" is already palindrome we do not need any insertions.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "mbadm"
 * Output: 2
 * Explanation: String can be "mbdadbm" or "mdbabdm".
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "leetcode"
 * Output: 5
 * Explanation: Inserting 5 characters the string becomes "leetcodocteel".
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 500
 *     s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn min_insertions(s: String) -> i32 {
		Self::solve(s.as_bytes(), 0, &mut vec![vec![-1; s.len() + 1]; s.len()])
	}

	fn solve(s: &[u8], offset: u16, cache: &mut [Vec<i32>]) -> i32 {
		match s {
			[b, ss @ .., e] => {
				if cache[offset as usize][s.len()] >= 0 {
					return cache[offset as usize][s.len()];
				}
				let mut ans;
				if b == e {
					ans = Self::solve(ss, offset + 1, cache)
				} else {
					ans = Self::solve(&s[1..], offset + 1, cache) + 1;
					ans = ans.min(Self::solve(&s[..s.len() - 1], offset, cache) + 1);
				}
				cache[offset as usize][s.len()] = ans;
				ans
			}
			_ => 0,
		}
	}
}

// submission codes end
