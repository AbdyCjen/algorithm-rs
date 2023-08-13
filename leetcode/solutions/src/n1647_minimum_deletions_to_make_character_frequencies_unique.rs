/**
 * [1647] Minimum Deletions to Make Character Frequencies Unique
 *
 * A string s is called good if there are no two different characters in s that have the same frequency.
 * Given a string s, return the minimum number of characters you need to delete to make s good.
 * The frequency of a character in a string is the number of times it appears in the string. For example, in the string "aab", the frequency of 'a' is 2, while the frequency of 'b' is 1.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "aab"
 * Output: 0
 * Explanation: s is already good.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "aaabbbcc"
 * Output: 2
 * Explanation: You can delete two 'b's resulting in the good string "aaabcc".
 * Another way it to delete one 'b' and one 'c' resulting in the good string "aaabbc".
 * <strong class="example">Example 3:
 *
 * Input: s = "ceabaacb"
 * Output: 2
 * Explanation: You can delete both 'c's resulting in the good string "eabaab".
 * Note that we only care about characters that are still in the string at the end (i.e. frequency of 0 is ignored).
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 10^5
 *     s contains only lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn min_deletions(s: String) -> i32 {
		let mut cnt = [0; 26];
		for c in s.bytes() {
			cnt[(c - b'a') as usize] += 1;
		}
		cnt.sort_unstable();
		let mut ans = 0;
		let (mut prv, mut avail) = (0, std::collections::VecDeque::new());
		for &i in cnt.iter().filter(|i| **i > 0) {
			if i == prv {
				ans += i - avail.pop_back().unwrap_or(0);
			}
			avail.extend(prv + 1..i);
			prv = i;
		}
		ans
	}
}

// submission codes end
