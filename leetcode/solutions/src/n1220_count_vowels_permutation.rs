/**
 * [1220] Count Vowels Permutation
 *
 * Given an integer n, your task is to count how many strings of length n can be formed under the following rules:
 *
 *     Each character is a lower case vowel ('a', 'e', 'i', 'o', 'u')
 *     Each vowel 'a' may only be followed by an 'e'.
 *     Each vowel 'e' may only be followed by an 'a' or an 'i'.
 *     Each vowel 'i' may not be followed by another 'i'.
 *     Each vowel 'o' may only be followed by an 'i' or a 'u'.
 *     Each vowel 'u' may only be followed by an 'a'.
 *
 * Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 1
 * Output: 5
 * Explanation: All possible strings are: "a", "e", "i" , "o" and "u".
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 2
 * Output: 10
 * Explanation: All possible strings are: "ae", "ea", "ei", "ia", "ie", "io", "iu", "oi", "ou" and "ua".
 *
 * <strong class="example">Example 3:
 *
 * Input: n = 5
 * Output: 68
 *  
 * Constraints:
 *
 *     1 <= n <= 2 * 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn count_vowel_permutation(n: i32) -> i32 {
		const FOLLOW: [&[usize]; 5] = [&[1], &[0, 2], &[0, 1, 3, 4], &[2, 4], &[0]];
		let mut cur = [1; 5];
		const MO: i32 = 1e9 as i32 + 7;
		for _ in 1..n {
			let mut new = [0; 5];
			for (i, n) in (0..).zip(cur) {
				for &fol in FOLLOW[i] {
					new[fol] = (new[fol] + n) % MO;
				}
			}
			cur = new;
		}
		cur.iter().fold(0, |a, n| (a + *n) % MO)
	}
}

// submission codes end
