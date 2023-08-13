/**
 * [1095] Find in Mountain Array
 *
 * (This problem is an interactive problem.)
 * You may recall that an array arr is a mountain array if and only if:
 *
 *     arr.length >= 3
 *     There exists some i with 0 < i < arr.length - 1 such that:
 *     
 *         arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
 *         arr[i] > arr[i + 1] > ... > arr[arr.length - 1]
 *     
 *     
 *
 * Given a mountain array mountainArr, return the minimum index such that mountainArr.get(index) == target. If such an index does not exist, return -1.
 * You cannot access the mountain array directly. You may only access the array using a MountainArray interface:
 *
 *     MountainArray.get(k) returns the element of the array at index k (0-indexed).
 *     MountainArray.length() returns the length of the array.
 *
 * Submissions making more than 100 calls to MountainArray.get will be judged Wrong Answer. Also, any solutions that attempt to circumvent the judge will result in disqualification.
 *  
 * <strong class="example">Example 1:
 *
 * Input: array = [1,2,3,4,5,3,1], target = 3
 * Output: 2
 * Explanation: 3 exists in the array, at index=2 and index=5. Return the minimum index, which is 2.
 * <strong class="example">Example 2:
 *
 * Input: array = [0,1,2,4,2,1], target = 3
 * Output: -1
 * Explanation: 3 does not exist in the array, so we return -1.
 *
 *  
 * Constraints:
 *
 *     3 <= mountain_arr.length() <= 10^4
 *     0 <= target <= 10^9
 *     0 <= mountain_arr.get(index) <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

/**
 * // This is the MountainArray's API interface.
 * // You should not implement it, or speculate about its implementation
 *  struct MountainArray;
 *  impl MountainArray {
 *     fn get(index:i32)->i32;
 *     fn length()->i32;
 * };
 */

pub struct MountainArray([i32]);
impl MountainArray {
	fn get(&self, idx: i32) -> i32 { self.0[idx as usize] }
	fn length(&self) -> i32 { self.0.len() as i32 }
}

impl Solution {
	pub fn find_in_mountain_array(target: i32, arr: &MountainArray) -> i32 {
		let (mut l, mut r) = (0, arr.length() - 1);
		while l < r {
			let mid = (l + r) / 2;
			if arr.get(mid) > arr.get(mid + 1) {
				r = mid;
			} else {
				l = mid + 1;
			}
		}
		let peak = l;
		let (mut l, mut r) = (0, peak + 1);
		use std::cmp::Ordering::*;
		while l < r {
			let mid = (l + r) / 2;
			match arr.get(mid).cmp(&target) {
				Equal => return mid,
				Greater => r = mid,
				Less => l = mid + 1,
			}
		}
		let (mut l, mut r) = (peak, arr.length());
		while l < r {
			let mid = (l + r) / 2;
			match arr.get(mid).cmp(&target) {
				Equal => return mid,
				Less => r = mid,
				Greater => l = mid + 1,
			}
		}
		-1
	}
}

// submission codes end
