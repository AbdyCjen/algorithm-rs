/**
 * [301] Remove Invalid Parentheses
 *
 * Given a string s that contains parentheses and letters, remove the minimum number of invalid parentheses to make the input string valid.
 * Return a list of unique strings that are valid with the minimum number of removals. You may return the answer in any order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "()())()"
 * Output: ["(())()","()()()"]
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "(a)())()"
 * Output: ["(a())()","(a)()()"]
 *
 * <strong class="example">Example 3:
 *
 * Input: s = ")("
 * Output: [""]
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 25
 *     s consists of lowercase English letters and parentheses '(' and ')'.
 *     There will be at most 20 parentheses in s.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	// FIXME: too dirty
	pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
		let mut ans = vec![];
		Self::solve(s.as_bytes(), &mut "".to_owned(), &mut ans, 0);
		ans.sort_unstable_by_key(|s| -(s.len() as i32));
		let l = ans[0].len();
		ans.truncate(ans.partition_point(|s| s.len() == l));
		let ans = ans.into_iter().collect::<std::collections::HashSet<_>>();
		ans.into_iter().collect()
	}
	fn solve(s: &[u8], cur: &mut String, ans: &mut Vec<String>, cnt: u8) {
		match s {
			[b'(', rest @ ..] => {
				Self::solve(rest, cur, ans, cnt);
				cur.push('(');
				Self::solve(rest, cur, ans, cnt + 1);
				cur.pop();
			}
			[b')', rest @ ..] => {
				Self::solve(rest, cur, ans, cnt);
				if let Some(cnt) = cnt.checked_sub(1) {
					cur.push(')');
					Self::solve(rest, cur, ans, cnt);
					cur.pop();
				}
			}
			[c, rest @ ..] => {
				cur.push(*c as char);
				Self::solve(rest, cur, ans, cnt);
				cur.pop();
			}
			// empty
			[] if cnt == 0 => ans.push(cur.clone()),
			[] => {}
		}
	}
}
