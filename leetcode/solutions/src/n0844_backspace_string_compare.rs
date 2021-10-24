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

#[allow(dead_code)]
impl Solution {
	pub fn backspace_compare(s: String, t: String) -> bool {
		fn filter_bs() -> impl FnMut(&u8) -> bool {
			let mut bs_cnt = 0;
			move |c| match (c, bs_cnt) {
				(&b'#', _) => {
					bs_cnt += 1;
					false
				}
				(_, __x @ 1..=std::i32::MAX) => {
					bs_cnt -= 1;
					false
				}
				_ => true,
			}
		}
		let mut s = s.bytes().rev().filter(filter_bs());
		let mut t = t.bytes().rev().filter(filter_bs());
		loop {
			match (s.next(), t.next()) {
				(Some(c1), Some(c2)) => {
					if c1 != c2 {
						break false;
					}
				}
				(None, None) => break true,
				_ => break false,
			}
		}
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
