/**
 * [1416] Restore The Array
 *
 * A program was supposed to print an array of integers. The program forgot to print whitespaces and the array is printed as a string of digits s and all we know is that all integers in the array were in the range [1, k] and there are no leading zeros in the array.
 * Given the string s and the integer k, return the number of the possible arrays that can be printed as s using the mentioned program. Since the answer may be very large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "1000", k = 10000
 * Output: 1
 * Explanation: The only possible array is [1000]
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "1000", k = 10
 * Output: 0
 * Explanation: There cannot be an array that was printed this way and has all integer >= 1 and <= 10.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "1317", k = 2000
 * Output: 8
 * Explanation: Possible arrays are [1317],[131,7],[13,17],[1,317],[13,1,7],[1,31,7],[1,3,17],[1,3,1,7]
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 10^5
 *     s consists of only digits and does not contain leading zeros.
 *     1 <= k <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn number_of_arrays(s: String, k: i32) -> i32 {
		Self::solve(s.as_bytes(), k, &mut vec![-1; s.len() + 1])
	}
	fn solve(s: &[u8], k: i32, cache: &mut [i32]) -> i32 {
		if cache[s.len()] >= 0 {
			return cache[s.len()];
		}
		match s {
			[c, ..] if *c == b'0' => 0,
			[] => 1,
			_ => {
				let (mut n, mut ans) = (0_i32, 0);
				let mut ss = s;
				while let [c, sss @ ..] = ss {
					n = n.saturating_mul(10).saturating_add((c - b'0') as i32);
					if n > k {
						break;
					}
					ans = (ans + Self::solve(sss, k, cache)) % (1e9 as i32 + 7);
					ss = sss;
				}
				cache[s.len()] = ans;
				ans
			}
		}
	}
}

// submission codes end
