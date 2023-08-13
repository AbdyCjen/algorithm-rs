/**
 * [1187] Make Array Strictly Increasing
 *
 * Given two integer arrays arr1 and arr2, return the minimum number of operations (possibly zero) needed to make arr1 strictly increasing.
 *
 * In one operation, you can choose two indices 0 <= i < arr1.length and 0 <= j < arr2.length and do the assignment arr1[i] = arr2[j].
 *
 * If there is no way to make arr1 strictly increasing, return -1.
 *
 *  
 * <strong class="example">Example 1:
 *
 *
 * Input: arr1 = [1,5,3,6,7], arr2 = [1,3,2,4]
 * Output: 1
 * Explanation: Replace 5 with 2, then arr1 = [1, 2, 3, 6, 7].
 *
 *
 * <strong class="example">Example 2:
 *
 *
 * Input: arr1 = [1,5,3,6,7], arr2 = [1, 3, 4]
 * Output: 2
 * Explanation: Replace 5 with 3 and then replace 3 with 4. arr1 = [1, 3, 4, 6, 7].
 *
 *
 * <strong class="example">Example 3:
 *
 *
 * Input: arr1 = [1,5,3,6,7], arr2 = [1,6,3,3]
 * Output: -1
 * Explanation: You can't make arr1 strictly increasing.
 *
 *  
 * Constraints:
 *
 *
 *     1 <= arr1.length, arr2.length <= 2000
 *     0 <= arr1[i], arr2[i] <= 10^9
 *
 *
 *  
 */
pub struct Solution {}

// submission codes start here

// TODO: simpler dp
use std::collections::HashMap;
impl Solution {
	pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
		arr2.sort_unstable();
		arr2.dedup();
		match Self::solve(&arr1, &arr2, -1, &mut HashMap::new()) {
			i32::MAX => -1,
			n => n,
		}
	}
	fn solve(
		arr1: &[i32],
		arr2: &[i32],
		prv: i32,
		cache: &mut HashMap<(i32, usize, usize), i32>,
	) -> i32 {
		if let Some(ans) = cache.get(&(prv, arr1.len(), arr2.len())) {
			return *ans;
		}
		let ans = match (arr1, arr2) {
			([n, ..], _) => {
				let mut ans = i32::MAX;
				if *n > prv {
					ans = ans.min(Self::solve(&arr1[1..], arr2, *n, cache));
				}
				let i = arr2.partition_point(|&i| i <= prv);
				if let Some(cand) = arr2.get(i) {
					ans = ans.min(
						Self::solve(&arr1[1..], &arr2[i + 1..], *cand, cache).saturating_add(1),
					);
				}
				ans
			}
			_ => 0,
		};
		cache.insert((prv, arr1.len(), arr2.len()), ans);
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1187() {
		assert_eq!(
			Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 4, 3]),
			2
		);
	}
}
