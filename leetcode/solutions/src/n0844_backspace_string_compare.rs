/**
 * [874] Backspace String Compare
 *
 * Given two strings S and T, return if they are equal when both are typed into empty text editors. # means a backspace character.
 *
 * <div>
 * Example 1:
 *
 *
 * Input: S = <span id="example-input-1-1">"ab#c"</span>, T = <span id="example-input-1-2">"ad#c"</span>
 * Output: <span id="example-output-1">true
 * </span><span>Explanation: Both S and T become "ac".</span>
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: S = <span id="example-input-2-1">"ab##"</span>, T = <span id="example-input-2-2">"c#d#"</span>
 * Output: <span id="example-output-2">true
 * </span><span>Explanation: Both S and T become "".</span>
 *
 *
 * <div>
 * Example 3:
 *
 *
 * Input: S = <span id="example-input-3-1">"a##c"</span>, T = <span id="example-input-3-2">"#a#c"</span>
 * Output: <span id="example-output-3">true
 * </span><span>Explanation: Both S and T become "c".</span>
 *
 *
 * <div>
 * Example 4:
 *
 *
 * Input: S = <span id="example-input-4-1">"a#c"</span>, T = <span id="example-input-4-2">"b"</span>
 * Output: <span id="example-output-4">false
 * </span><span>Explanation: S becomes "c" while T becomes "b".</span>
 *
 *
 * <span>Note:</span>
 *
 * <ol>
 * <span>1 <= S.length <= 200</span>
 * <span>1 <= T.length <= 200</span>
 * <span>S and T only contain lowercase letters and '#' characters.</span>
 * </ol>
 *
 * Follow up:
 *
 *
 * Can you solve it in O(N) time and O(1) space?
 *
 * </div>
 * </div>
 * </div>
 * </div>
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn backspace_compare(s: String, t: String) -> bool {
		Self::solve(s.into_bytes()) == Self::solve(t.into_bytes())
	}
	fn solve(mut s: Vec<u8>) -> Vec<u8> {
		let mut i = 0_usize;
		for j in 0..s.len() {
			match s[j] {
				b'#' => i = i.saturating_sub(1),
				c => {
					s[i] = c;
					i += 1;
				}
			}
		}
		s.truncate(i);
		s
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_874() {
		assert!(Solution::backspace_compare(
			"ab#c".to_owned(),
			"ad#c".to_owned()
		));
		assert!(Solution::backspace_compare(
			"ab##".to_owned(),
			"c#d#".to_owned()
		));
		assert!(Solution::backspace_compare(
			"a##c".to_owned(),
			"#a#c".to_owned()
		));
		assert!(!Solution::backspace_compare("a#c".into(), "b".into()));
		assert!(!Solution::backspace_compare(
			"bxj##tw".to_owned(),
			"bxj###tw".to_owned()
		));
	}
}
