/**
 * [169] Majority Element
 *
 * Given an array of size n, find the majority element. The majority element is the element that appears more than &lfloor; n/2 &rfloor; times.
 *
 * You may assume that the array is non-empty and the majority element always exist in the array.
 *
 * Example 1:
 *
 *
 * Input: [3,2,3]
 * Output: 3
 *
 * Example 2:
 *
 *
 * Input: [2,2,1,1,1,2,2]
 * Output: 2
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn majority_element(nums: Vec<i32>) -> i32 { Self::candidate(nums.into_iter()) }
	fn candidate(mut nums: impl Iterator<Item = i32>) -> i32 {
		let (mut cnt, tar) = (1, nums.next().unwrap());
		while cnt > 0 {
			match nums.next() {
				Some(o) if o == tar => cnt += 1,
				Some(_) => cnt -= 1,
				None => return tar,
			}
		}
		Self::candidate(nums)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_169() {
		assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
		assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
	}
}
