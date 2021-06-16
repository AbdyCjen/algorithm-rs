/**
 * [84] Largest Rectangle in Histogram
 *
 * Given n non-negative integers representing the histogram's bar height where the width of each bar is 1, find the area of largest rectangle in the histogram.
 *
 *  
 *
 * <img src="https://assets.leetcode.com/uploads/2018/10/12/histogram.png" style="width: 188px; height: 204px;" /><br />
 * <small>Above is a histogram where width of each bar is 1, given height = [2,1,5,6,2,3].</small>
 *
 *  
 *
 * <img src="https://assets.leetcode.com/uploads/2018/10/12/histogram_area.png" style="width: 188px; height: 204px;" /><br />
 * <small>The largest rectangle is shown in the shaded area, which has area = 10 unit.</small>
 *
 *  
 *
 * Example:
 *
 *
 * Input: [2,1,5,6,2,3]
 * Output: 10
 *
 *
 */
pub struct Solution {}

// submission codes start here

/*
  看了笛卡尔树的构建才知道咋回事, 我擦泪;
  算法主流程基本等价于小根笛卡尔树的构建;
  从左向右扫描输入, 维护一个栈st表示扫描到当前位置i时,
*/
#[allow(dead_code)]
impl Solution {
	pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
		if heights.is_empty() {
			return 0;
		}
		let mut st = vec![(0, heights[0])];
		let mut max_area: i32 = heights[0];
		heights.push(0);
		for (i, &h) in heights.iter().enumerate().skip(1) {
			while let Some(&(_, th)) = st.last() {
				if th < h {
					break;
				}
				st.pop();
				max_area = match st.last() {
					None => max_area.max(i as i32 * th),
					Some(&(ni, _)) => max_area.max((i - ni - 1) as i32 * th),
				}
			}
			st.push((i, h));
		}
		max_area
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_84() {
		assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
		assert_eq!(Solution::largest_rectangle_area(vec![]), 0);
		assert_eq!(Solution::largest_rectangle_area(vec![8]), 8);
		assert_eq!(Solution::largest_rectangle_area(vec![0, 9]), 9);
		assert_eq!(Solution::largest_rectangle_area(vec![0, 0, 0, 9, 0]), 9);
		assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 1, 1, 2]), 5);
		assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 4, 5]), 9);
	}
}
