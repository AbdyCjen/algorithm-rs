/**
 * [282] Expression Add Operators
 *
 * Given a string num that contains only digits and an integer target, return all possibilities to add the binary operators '+', '-', or '*' between the digits of num so that the resultant expression evaluates to the target value.
 *  
 * Example 1:
 * Input: num = "123", target = 6
 * Output: ["1*2*3","1+2+3"]
 * Example 2:
 * Input: num = "232", target = 8
 * Output: ["2*3+2","2+3*2"]
 * Example 3:
 * Input: num = "105", target = 5
 * Output: ["1*0+5","10-5"]
 * Example 4:
 * Input: num = "00", target = 0
 * Output: ["0*0","0+0","0-0"]
 * Example 5:
 * Input: num = "3456237490", target = 9191
 * Output: []
 *  
 * Constraints:
 *
 *     1 <= num.length <= 10
 *     num consists of only digits.
 *     -2^31 <= target <= 2^31 - 1
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn add_operators(num: String, target: i32) -> Vec<String> {
		fn solution(
			num: &[u8],
			expr: String,
			ans: &mut Vec<String>,
			target: i32,
			eval: i64,
			prev: i64,
		) {
			if num.is_empty() {
				if target as i64 == eval + prev {
					ans.push(expr);
				}
				return;
			}

			let mut cur = 0;
			for (i, &c) in num.iter().enumerate() {
				if i != 0 && num[0] == b'0' {
					break;
				}
				cur = cur * 10 + (c - b'0') as i64;
				let cur_str = unsafe { std::str::from_utf8_unchecked(&num[..i + 1]) };
				if expr.is_empty() {
					solution(&num[i + 1..], expr.clone() + cur_str, ans, target, 0, cur);
				} else {
					solution(
						&num[i + 1..],
						expr.clone() + "+" + cur_str,
						ans,
						target,
						eval + prev,
						cur,
					);
					solution(
						&num[i + 1..],
						expr.clone() + "-" + cur_str,
						ans,
						target,
						eval + prev,
						-cur,
					);
					solution(
						&num[i + 1..],
						expr.clone() + "*" + cur_str,
						ans,
						target,
						eval,
						prev * cur,
					);
				}
			}
		}

		let mut ans = vec![];
		solution(num.as_bytes(), "".to_owned(), &mut ans, target, 0, 0);
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_282() {
		let test_fn = |a1: Vec<String>, a2: Vec<String>| {
			use std::collections::HashSet;
			let a1: HashSet<_> = a1.into_iter().collect();
			let a2: HashSet<_> = a2.into_iter().collect();
			assert_eq!(a1, a2);
		};
		test_fn(
			Solution::add_operators("123".to_owned(), 6),
			vec_string!["1+2+3", "1*2*3"],
		);
		test_fn(
			Solution::add_operators("232".to_owned(), 8),
			vec_string!["2*3+2", "2+3*2"],
		);
		test_fn(
			Solution::add_operators("105".to_owned(), 5),
			vec_string!["1*0+5", "10-5"],
		);
		test_fn(
			Solution::add_operators("00".to_owned(), 0),
			vec_string!["0*0", "0+0", "0-0"],
		);
		test_fn(
			Solution::add_operators("3456237490".to_owned(), 9191),
			vec!["".to_owned(); 0],
		);
	}
}
