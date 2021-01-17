/**
 * [424] Longest Repeating Character Replacement
 *
 * Given a string s that consists of only uppercase English letters, you can perform at most k operations on that string.
 *
 * In one operation, you can choose any character of the string and change it to any other uppercase English character.
 *
 * Find the length of the longest sub-string containing all repeating letters you can get after performing the above operations.
 *
 * Note:<br />
 * Both the string's length and k will not exceed 10^4.
 *
 * Example 1:
 *
 *
 * Input:
 * s = "ABAB", k = 2
 *
 * Output:
 * 4
 *
 * Explanation:
 * Replace the two 'A's with two 'B's or vice versa.
 *
 *
 *  
 *
 * Example 2:
 *
 *
 * Input:
 * s = "AABABBA", k = 1
 *
 * Output:
 * 4
 *
 * Explanation:
 * Replace the one 'A' in the middle with 'B' and form "AABBBBA".
 * The substring "BBBB" has the longest repeating letters, which is 4.
 *
 *
 *  
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn character_replacement(s: String, k: i32) -> i32 {
		let s = s.into_bytes();
		let mut cnt = [0; 26];
		let mut i = 0;
		for (j, c) in s.iter().copied().enumerate() {
			cnt[(c - b'A') as usize] += 1;
			if cnt.iter().max().unwrap() + k < (j - i + 1) as i32 {
				cnt[(s[i] - b'A') as usize] -= 1;
				i += 1;
			}
		}
		(s.len() - i) as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_424() {
		assert_eq!(Solution::character_replacement("ABAB".to_owned(), 2), 4);
		assert_eq!(Solution::character_replacement("AABABBA".to_owned(), 1), 4);
	}
}
