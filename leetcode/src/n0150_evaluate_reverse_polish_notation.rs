/**
 * [150] Evaluate Reverse Polish Notation
 *
 * Evaluate the value of an arithmetic expression in <a href="http://en.wikipedia.org/wiki/Reverse_Polish_notation" target="_blank">Reverse Polish Notation</a>.
 *
 * Valid operators are +, -, *, /. Each operand may be an integer or another expression.
 *
 * Note:
 *
 *
 * Division between two integers should truncate toward zero.
 * The given RPN expression is always valid. That means the expression would always evaluate to a result and there won't be any divide by zero operation.
 *
 *
 * Example 1:
 *
 *
 * Input: ["2", "1", "+", "3", "*"]
 * Output: 9
 * Explanation: ((2 + 1) * 3) = 9
 *
 *
 * Example 2:
 *
 *
 * Input: ["4", "13", "5", "/", "+"]
 * Output: 6
 * Explanation: (4 + (13 / 5)) = 6
 *
 *
 * Example 3:
 *
 *
 * Input: ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
 * Output: 22
 * Explanation:
 *   ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
 * = ((10 * (6 / (12 * -11))) + 17) + 5
 * = ((10 * (6 / -132)) + 17) + 5
 * = ((10 * 0) + 17) + 5
 * = (0 + 17) + 5
 * = 17 + 5
 * = 22
 *
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn eval_rpn(tokens: Vec<String>) -> i32 {
		let mut st: Vec<i32> = vec![];

		for tok in tokens.iter() {
			if tok == "+" {
				let ro = st.pop().unwrap();
				let lo = st.pop().unwrap();
				st.push(lo + ro);
			} else if tok == "-" {
				let ro = st.pop().unwrap();
				let lo = st.pop().unwrap();
				st.push(lo - ro);
			} else if tok == "*" {
				let ro = st.pop().unwrap();
				let lo = st.pop().unwrap();
				st.push(lo * ro);
			} else if tok == "/" {
				let ro = st.pop().unwrap();
				let lo = st.pop().unwrap();
				st.push(lo / ro);
			} else {
				st.push(tok.parse::<i32>().unwrap());
			}
		}
		st[0]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_150() {
		assert_eq!(
			Solution::eval_rpn(
				vec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
					.iter()
					.map(|i| String::from(*i))
					.collect()
			),
			22
		);
	}
}
