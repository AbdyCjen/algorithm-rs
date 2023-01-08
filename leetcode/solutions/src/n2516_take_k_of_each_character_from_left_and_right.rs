/**
 * [2516] Take K of Each Character From Left and Right
 *
 * You are given a string s consisting of the characters 'a', 'b', and 'c' and a non-negative integer k. Each minute, you may take either the leftmost character of s, or the rightmost character of s.
 * Return the minimum number of minutes needed for you to take at least k of each character, or return -1 if it is not possible to take k of each character.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "aabaaaacaabc", k = 2
 * Output: 8
 * Explanation:
 * Take three characters from the left of s. You now have two 'a' characters, and one 'b' character.
 * Take five characters from the right of s. You now have four 'a' characters, two 'b' characters, and two 'c' characters.
 * A total of 3 + 5 = 8 minutes is needed.
 * It can be proven that 8 is the minimum number of minutes needed.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "a", k = 1
 * Output: -1
 * Explanation: It is not possible to take one 'b' or 'c' so return -1.
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 10^5
 *     s consists of only the letters 'a', 'b', and 'c'.
 *     0 <= k <= s.length
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn take_characters(s: String, k: i32) -> i32 {
		let mut cnt = [0; 3];
		let n = s.as_bytes().len();
		let mut idx = -1;
		for (i, c) in s.bytes().enumerate() {
			cnt[(c - b'a') as usize] += 1;
			if cnt[0] >= k && cnt[1] >= k && cnt[2] >= k {
				idx = i as i32;
				break;
			}
		}
		if idx < 0 {
			return -1;
		}
		let mut ans = idx + 1;
		let mut j = n;
		for (i, c) in s.bytes().enumerate().take((idx + 1) as usize).rev() {
			cnt[(c - b'a') as usize] -= 1;
			if cnt[0] < k || cnt[1] < k || cnt[2] < k {
				j -= 1;
				let cc = s.as_bytes()[j];
				cnt[(cc - b'a') as usize] += 1;
			}
			if cnt[0] >= k && cnt[1] >= k && cnt[2] >= k {
				ans = ans.min((n - j + i) as i32);
			}
		}
		ans
	}
}

// submission codes end
