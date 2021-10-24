/**
 * [1446] Angle Between Hands of a Clock
 *
 * Given two numbers, hour and minutes. Return the smaller angle (in degrees) formed between the hour and the minute hand.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/12/26/sample_1_1673.png" style="width: 230px; height: 225px;" />
 *
 * Input: hour = 12, minutes = 30
 * Output: 165
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/12/26/sample_2_1673.png" style="width: 230px; height: 225px;" />
 *
 * Input: hour = 3, minutes = 30
 * Output: 75
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/12/26/sample_3_1673.png" style="width: 230px; height: 225px;" />
 *
 * Input: hour = 3, minutes = 15
 * Output: 7.5
 *
 * Example 4:
 *
 * Input: hour = 4, minutes = 50
 * Output: 155
 *
 * Example 5:
 *
 * Input: hour = 12, minutes = 0
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     1 <= hour <= 12
 *     0 <= minutes <= 59
 *     Answers within 10^-5 of the actual value will be accepted as correct.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
		let hour_offset = hour * 60 + minutes;
		let minute_offset = minutes * 12;
		let angle_raw = (hour_offset - minute_offset).abs();

		let angle = if angle_raw > 6 * 60 {
			12 * 60 - angle_raw
		} else {
			angle_raw
		};

		(360 * angle) as f64 / (12 * 60) as f64
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[allow(clippy::float_cmp)]
	fn test_1446() {
		assert_eq!(Solution::angle_clock(12, 30), 165.0);
		assert_eq!(Solution::angle_clock(3, 30), 75.0);
		assert_eq!(Solution::angle_clock(3, 15), 7.5);
		assert_eq!(Solution::angle_clock(4, 50), 155.0);
		assert_eq!(Solution::angle_clock(12, 0), 0.0);
	}
}
