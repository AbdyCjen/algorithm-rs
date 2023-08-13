/**
 * [17] Letter Combinations of a Phone Number
 *
 * Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
 * A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/15/1200px-telephone-keypad2svg.png" style="width: 300px; height: 243px;" />
 *  
 * <strong class="example">Example 1:
 *
 * Input: digits = "23"
 * Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
 *
 * <strong class="example">Example 2:
 *
 * Input: digits = ""
 * Output: []
 *
 * <strong class="example">Example 3:
 *
 * Input: digits = "2"
 * Output: ["a","b","c"]
 *
 *  
 * Constraints:
 *
 *     0 <= digits.length <= 4
 *     digits[i] is a digit in the range ['2', '9'].
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	const DIST: [&'static str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
	pub fn letter_combinations(digits: String) -> Vec<String> {
		if digits.is_empty() {
			return vec![];
		}
		let mut ans = vec![];
		Self::solve(String::new(), digits.as_bytes(), &mut ans);
		ans
	}
	fn solve(st: String, digit: &[u8], ans: &mut Vec<String>) {
		match digit {
			[] => ans.push(st),
			[c, rest @ ..] => {
				for c in Self::DIST[(c - b'2') as usize].chars() {
					let mut st = st.clone();
					st.push(c);
					Self::solve(st, rest, ans);
				}
			}
		}
	}
}

// submission codes end
