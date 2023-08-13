/**
 * [97] Interleaving String
 *
 * Given strings s1, s2, and s3, find whether s3 is formed by an interleaving of s1 and s2.
 * An interleaving of two strings s and t is a configuration where s and t are divided into n and m <span data-keyword="substring-nonempty">substrings</span> respectively, such that:
 *
 *     s = s1 + s2 + ... + sn
 *     t = t1 + t2 + ... + tm
 *     |n - m| <= 1
 *     The interleaving is s1 + t1 + s2 + t2 + s3 + t3 + ... or t1 + s1 + t2 + s2 + t3 + s3 + ...
 *
 * Note: a + b is the concatenation of strings a and b.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/02/interleave.jpg" style="width: 561px; height: 203px;" />
 * Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
 * Output: true
 * Explanation: One way to obtain s3 is:
 * Split s1 into s1 = "aa" + "bc" + "c", and s2 into s2 = "dbbc" + "a".
 * Interleaving the two splits, we get "aa" + "dbbc" + "bc" + "a" + "c" = "aadbbcbcac".
 * Since s3 can be obtained by interleaving s1 and s2, we return true.
 *
 * <strong class="example">Example 2:
 *
 * Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
 * Output: false
 * Explanation: Notice how it is impossible to interleave s2 with any other string to obtain s3.
 *
 * <strong class="example">Example 3:
 *
 * Input: s1 = "", s2 = "", s3 = ""
 * Output: true
 *
 *  
 * Constraints:
 *
 *     0 <= s1.length, s2.length <= 100
 *     0 <= s3.length <= 200
 *     s1, s2, and s3 consist of lowercase English letters.
 *
 *  
 * Follow up: Could you solve it using only O(s2.length) additional memory space?
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
		s1.len() + s2.len() == s3.len()
			&& Self::solve(
				s1.as_bytes(),
				s2.as_bytes(),
				s3.as_bytes(),
				&mut vec![vec![None; s2.len() + 1]; s1.len() + 1],
			)
	}
	fn solve(s1: &[u8], s2: &[u8], s3: &[u8], cache: &mut [Vec<Option<bool>>]) -> bool {
		if let Some(ans) = cache[s1.len()][s2.len()] {
			return ans;
		}
		let ans = match (s1, s2, s3) {
			(_, _, []) => true,
			([c1, rest1 @ ..], [c2, rest2 @ ..], [c3, s3 @ ..]) => {
				(c1 == c3 && Self::solve(rest1, s2, s3, cache))
					|| (c2 == c3 && Self::solve(s1, rest2, s3, cache))
			}
			(s, [], s3) | ([], s, s3) => s3 == s,
		};
		cache[s1.len()][s2.len()] = Some(ans);
		ans
	}
}

// submission codes end
