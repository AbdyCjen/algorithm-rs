/**
 * [739] Daily Temperatures
 *
 * Given a list of daily temperatures temperatures, return a list such that, for each day in the input, tells you how many days you would have to wait until a warmer temperature. If there is no future day for which this is possible, put 0 instead.
 * For example, given the list of temperatures temperatures = [73, 74, 75, 71, 69, 72, 76, 73], your output should be [1, 1, 4, 2, 1, 1, 0, 0].
 * Note: The length of temperatures will be in the range [1, 30000]. Each temperature will be an integer in the range [30, 100].
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn daily_temperatures(temps: Vec<i32>) -> Vec<i32> {
		let mut ans = vec![0; temps.len()];
		let mut st = vec![];
		for (i, t) in temps.into_iter().enumerate() {
			while let Some(&(ii, tt)) = st.last() {
				if tt < t {
					ans[ii] = (i - ii) as i32;
					st.pop();
				} else {
					break;
				}
			}
			st.push((i, t));
		}
		ans
	}
	pub fn daily_temperatures_1(mut temps: Vec<i32>) -> Vec<i32> {
		let mut mono_stk = vec![(0, temps.pop().unwrap())];
		let mut ans = vec![0];

		for (i, t) in temps.into_iter().rev().enumerate() {
			while mono_stk.last().map(|&(_, t2)| t2 <= t).unwrap_or(false) {
				mono_stk.pop();
			}

			ans.push(
				mono_stk
					.last()
					.map(|&(j, _)| (i - j) as i32 + 1)
					.unwrap_or(0),
			);
			mono_stk.push((i + 1, t));
		}
		ans.reverse();
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_739() {
		assert_eq!(
			Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
			vec![1, 1, 4, 2, 1, 1, 0, 0]
		);

		assert_eq!(
			Solution::daily_temperatures(vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70]),
			vec![8, 1, 5, 4, 3, 2, 1, 1, 0, 0]
		);
	}
}
