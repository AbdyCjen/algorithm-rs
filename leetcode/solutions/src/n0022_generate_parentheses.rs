/**
 * [22] Generate Parentheses
 *
 * Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
 *  
 * Example 1:
 * Input: n = 3
 * Output: ["((()))","(()())","(())()","()(())","()()()"]
 * Example 2:
 * Input: n = 1
 * Output: ["()"]
 *  
 * Constraints:
 *
 *     1 <= n <= 8
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn generate_parenthesis(n: i32) -> Vec<String> {
		fn generate_inner(l: i32, r: i32, mut st: Vec<u8>, ans: &mut Vec<String>) {
			if l == 0 {
				if r > 0 {
					st.extend(std::iter::repeat(b')').take(r as usize));
				}
				unsafe {
					ans.push(String::from_utf8_unchecked(st));
				}
				return;
			}
			for i in 0..=r {
				let mut st_new = st.clone();
				st_new.push(b'(');
				generate_inner(l - 1, r + 1 - i, st_new, ans);
				st.push(b')');
			}
		}
		let mut ans = Vec::new();
		generate_inner(n, 0, Vec::new(), &mut ans);
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_22() {
		assert_eq!(
			Solution::generate_parenthesis(3),
			vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
		);
		assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
	}
}
