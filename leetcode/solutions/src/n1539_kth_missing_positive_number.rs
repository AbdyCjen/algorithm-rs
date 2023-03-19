/**
 * [1539] Kth Missing Positive Number
 *
 * Given an array arr of positive integers sorted in a strictly increasing order, and an integer k.
 * Return the k^th positive integer that is missing from this array.
 *  
 * <strong class="example">Example 1:
 *
 * Input: arr = [2,3,4,7,11], k = 5
 * Output: 9
 * Explanation: The missing positive integers are [1,5,6,8,9,10,12,13,...]. The 5^th missing positive integer is 9.
 *
 * <strong class="example">Example 2:
 *
 * Input: arr = [1,2,3,4], k = 2
 * Output: 6
 * Explanation: The missing positive integers are [5,6,7,...]. The 2^nd missing positive integer is 6.
 *
 *  
 * Constraints:
 *
 *     1 <= arr.length <= 1000
 *     1 <= arr[i] <= 1000
 *     1 <= k <= 1000
 *     arr[i] < arr[j] for 1 <= i < j <= arr.length
 *
 *  
 * Follow up:
 * Could you solve this problem in less than O(n) complexity?
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	//TODO: binary search
	pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
		for (&n, i) in arr.iter().zip(1..) {
			if n - i >= k {
				return k + i - 1;
			}
		}
		k + arr.len() as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1539() {
		assert_eq!(Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5), 9);
		assert_eq!(Solution::find_kth_positive(vec![1, 2, 3, 4], 2), 6);
	}
}
