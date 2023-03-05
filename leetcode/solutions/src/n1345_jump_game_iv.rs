/**
 * [1345] Jump Game IV
 *
 * Given an array of integers arr, you are initially positioned at the first index of the array.
 * In one step you can jump from index i to index:
 *
 *     i + 1 where: i + 1 < arr.length.
 *     i - 1 where: i - 1 >= 0.
 *     j where: arr[i] == arr[j] and i != j.
 *
 * Return the minimum number of steps to reach the last index of the array.
 * Notice that you can not jump outside of the array at any time.
 *  
 * <strong class="example">Example 1:
 *
 * Input: arr = [100,-23,-23,404,100,23,23,23,3,404]
 * Output: 3
 * Explanation: You need three jumps from index 0 --> 4 --> 3 --> 9. Note that index 9 is the last index of the array.
 *
 * <strong class="example">Example 2:
 *
 * Input: arr = [7]
 * Output: 0
 * Explanation: Start index is the last index. You do not need to jump.
 *
 * <strong class="example">Example 3:
 *
 * Input: arr = [7,6,9,6,9,6,9,7]
 * Output: 1
 * Explanation: You can jump directly from index 0 to index 7 which is last index of the array.
 *
 *  
 * Constraints:
 *
 *     1 <= arr.length <= 5 * 10^4
 *     -10^8 <= arr[i] <= 10^8
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn min_jumps(arr: Vec<i32>) -> i32 {
		let mut idxs = std::collections::HashMap::new();
		for (i, &n) in arr.iter().enumerate() {
			idxs.entry(n).or_insert_with(Vec::new).push(i);
		}
		let mut visited = vec![false; arr.len()];
		let mut dq = vec![0_usize];
		let mut step = 0;
		while !dq.is_empty() {
			for i in std::mem::take(&mut dq) {
				if i == arr.len() - 1 {
					return step;
				}
				let nei = [i.saturating_sub(1), (i + 1).min(arr.len() - 1)];
				for n in idxs.remove(&arr[i]).into_iter().flatten().chain(nei) {
					if !visited[n] {
						dq.push(n);
						visited[n] = true;
					}
				}
			}
			step += 1;
		}

		step
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1345() {
		assert_eq!(Solution::min_jumps(vec![7]), 0);
		assert_eq!(
			Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]),
			3
		);
		assert_eq!(Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
		let mut arr = vec![7; 49999];
		arr.push(11);
		assert_eq!(Solution::min_jumps(arr), 2);
	}
}
