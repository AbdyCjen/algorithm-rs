/**
 * [2478] Number of Beautiful Partitions
 *
 * You are given a string s that consists of the digits '1' to '9' and two integers k and minLength.
 * A partition of s is called beautiful if:
 *
 *     s is partitioned into k non-intersecting substrings.
 *     Each substring has a length of at least minLength.
 *     Each substring starts with a prime digit and ends with a non-prime digit. Prime digits are '2', '3', '5', and '7', and the rest of the digits are non-prime.
 *
 * Return the number of beautiful partitions of s. Since the answer may be very large, return it modulo 10^9 + 7.
 * A substring is a contiguous sequence of characters within a string.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "23542185131", k = 3, minLength = 2
 * Output: 3
 * Explanation: There exists three ways to create a beautiful partition:
 * "2354 | 218 | 5131"
 * "2354 | 21851 | 31"
 * "2354218 | 51 | 31"
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "23542185131", k = 3, minLength = 3
 * Output: 1
 * Explanation: There exists one way to create a beautiful partition: "2354 | 218 | 5131".
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "3312958", k = 3, minLength = 1
 * Output: 1
 * Explanation: There exists one way to create a beautiful partition: "331 | 29 | 58".
 *
 *  
 * Constraints:
 *
 *     1 <= k, minLength <= s.length <= 1000
 *     s consists of the digits '1' to '9'.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	const PRIME: [u8; 4] = [b'2', b'3', b'5', b'7'];
	pub fn beautiful_partitions(s: String, k: i32, minl: i32) -> i32 {
		let s = s.into_bytes();
		if !Self::PRIME.contains(&s[0]) || Self::PRIME.contains(&s[s.len() - 1]) {
			return 0;
		}
		Self::solve(
			&s,
			k as usize,
			minl as usize,
			&mut vec![vec![-1; s.len() + 1]; 1 + k as usize],
		)
	}

	fn solve(s: &[u8], k: usize, minl: usize, cache: &mut [Vec<i32>]) -> i32 {
		const MO: i32 = 1e9 as i32 + 7;
		if cache[k][s.len()] >= 0 {
			return cache[k][s.len()];
		}

		match s {
			[] => (k == 0) as i32,
			[c, ..] if k == 0 || s.len() < minl || !Self::PRIME.contains(c) => 0,
			_ => {
				let mut ans = 0;
				for i in minl - 1..(s.len().saturating_sub(minl * (k - 1))) {
					if !Self::PRIME.contains(&s[i]) {
						ans = (ans + Self::solve(&s[i + 1..], k - 1, minl, cache)) % MO;
					}
				}
				cache[k][s.len()] = ans;
				ans
			}
		}
	}
}
