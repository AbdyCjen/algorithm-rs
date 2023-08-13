/**
 * [373] Find K Pairs with Smallest Sums
 *
 * You are given two integer arrays nums1 and nums2 sorted in ascending order and an integer k.
 * Define a pair (u, v) which consists of one element from the first array and one element from the second array.
 * Return the k pairs (u1, v1), (u2, v2), ..., (uk, vk) with the smallest sums.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums1 = [1,7,11], nums2 = [2,4,6], k = 3
 * Output: [[1,2],[1,4],[1,6]]
 * Explanation: The first 3 pairs are returned from the sequence: [1,2],[1,4],[1,6],[7,2],[7,4],[11,2],[7,6],[11,4],[11,6]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums1 = [1,1,2], nums2 = [1,2,3], k = 2
 * Output: [[1,1],[1,1]]
 * Explanation: The first 2 pairs are returned from the sequence: [1,1],[1,1],[1,2],[2,1],[1,2],[2,2],[1,3],[1,3],[2,3]
 *
 * <strong class="example">Example 3:
 *
 * Input: nums1 = [1,2], nums2 = [3], k = 3
 * Output: [[1,3],[2,3]]
 * Explanation: All possible pairs are returned from the sequence: [1,3],[2,3]
 *
 *  
 * Constraints:
 *
 *     1 <= nums1.length, nums2.length <= 10^5
 *     -10^9 <= nums1[i], nums2[i] <= 10^9
 *     nums1 and nums2 both are sorted in ascending order.
 *     1 <= k <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
		let mut bh: std::collections::BinaryHeap<_> = (0..)
			.zip(&nums1)
			.map(|(i, n)| (-n - nums2[0], i, 0))
			.collect();
		(0..k)
			.map_while(|_| {
				bh.pop().map(|(_, i, j)| {
					if j + 1 < nums2.len() {
						bh.push((-nums1[i] - nums2[j + 1], i, j + 1));
					}
					vec![nums1[i], nums2[j]]
				})
			})
			.collect()
	}
	pub fn k_smallest_pairs1(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
		let mut bh: std::collections::BinaryHeap<_> = vec![(-nums1[0] - nums2[0], 0, 0)].into();
		let mut visited = std::collections::HashSet::new();
		(0..k)
			.map_while(|_| {
				bh.pop().map(|(_, i, j)| {
					if i + 1 < nums1.len() && visited.insert((i + 1, j)) {
						bh.push((-nums1[i + 1] - nums2[j], i + 1, j));
					}
					if j + 1 < nums2.len() && visited.insert((i, j + 1)) {
						bh.push((-nums1[i] - nums2[j + 1], i, j + 1));
					}
					vec![nums1[i], nums2[j]]
				})
			})
			.collect()
	}
}

// submission codes end
