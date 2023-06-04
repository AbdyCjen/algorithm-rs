/**
 * [2215] Find the Difference of Two Arrays
 *
 * Given two 0-indexed integer arrays nums1 and nums2, return a list answer of size 2 where:
 *
 *     answer[0] is a list of all distinct integers in nums1 which are not present in nums2.
 *     answer[1] is a list of all distinct integers in nums2 which are not present in nums1.
 *
 * Note that the integers in the lists may be returned in any order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums1 = [1,2,3], nums2 = [2,4,6]
 * Output: [[1,3],[4,6]]
 * Explanation:
 * For nums1, nums1[1] = 2 is present at index 0 of nums2, whereas nums1[0] = 1 and nums1[2] = 3 are not present in nums2. Therefore, answer[0] = [1,3].
 * For nums2, nums2[0] = 2 is present at index 1 of nums1, whereas nums2[1] = 4 and nums2[2] = 6 are not present in nums2. Therefore, answer[1] = [4,6].
 * <strong class="example">Example 2:
 *
 * Input: nums1 = [1,2,3,3], nums2 = [1,1,2,2]
 * Output: [[3],[]]
 * Explanation:
 * For nums1, nums1[2] and nums1[3] are not present in nums2. Since nums1[2] == nums1[3], their value is only included once and answer[0] = [3].
 * Every integer in nums2 is present in nums1. Therefore, answer[1] = [].
 *
 *  
 * Constraints:
 *
 *     1 <= nums1.length, nums2.length <= 1000
 *     -1000 <= nums1[i], nums2[i] <= 1000
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	#[rustfmt::skip]
	pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
		let (mut cnt1, mut cnt2) = (vec![false; 2001], vec![false; 2001]);
		for i in nums1 { cnt1[(i + 1000) as usize] = true; }
		for i in nums2 { cnt2[(i + 1000) as usize] = true; }
		let mut ans = [vec![], vec![]];
		for ((f1, f2), i) in cnt1.into_iter().zip(cnt2).zip(-1000..) {
			match (f1, f2) {
				(true, false) => ans[0].push(i),
				(false, true) => ans[1].push(i),
				_ => {}
			}
		}
		ans.into()
	}

	pub fn find_difference1(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<Vec<i32>> {
		nums1.sort_unstable();
		nums1.dedup();
		nums2.sort_unstable();
		nums2.dedup();
		let mut ans = [vec![], vec![]];
		while let (Some(a), Some(b)) = (nums1.last(), nums2.last()) {
			#[allow(clippy::comparison_chain)]
			if a == b {
				nums1.pop();
				nums2.pop();
			} else if a < b {
				ans[1].push(*b);
				nums2.pop();
			} else {
				ans[0].push(*a);
				nums1.pop();
			}
		}
		ans[0].extend(nums1);
		ans[1].extend(nums2);
		ans.into()
	}
}

// submission codes end
