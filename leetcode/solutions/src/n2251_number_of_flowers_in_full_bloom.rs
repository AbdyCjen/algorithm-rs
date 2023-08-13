/**
 * [2251] Number of Flowers in Full Bloom
 *
 * You are given a 0-indexed 2D integer array flowers, where flowers[i] = [starti, endi] means the i^th flower will be in full bloom from starti to endi (inclusive). You are also given a 0-indexed integer array people of size n, where people[i] is the time that the i^th person will arrive to see the flowers.
 * Return an integer array answer of size n, where answer[i] is the number of flowers that are in full bloom when the i^th person arrives.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/02/ex1new.jpg" style="width: 550px; height: 216px;" />
 * Input: flowers = [[1,6],[3,7],[9,12],[4,13]], poeple = [2,3,7,11]
 * Output: [1,2,2,2]
 * Explanation: The figure above shows the times when the flowers are in full bloom and when the people arrive.
 * For each person, we return the number of flowers in full bloom during their arrival.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/02/ex2new.jpg" style="width: 450px; height: 195px;" />
 * Input: flowers = [[1,10],[3,3]], poeple = [3,3,2]
 * Output: [2,2,1]
 * Explanation: The figure above shows the times when the flowers are in full bloom and when the people arrive.
 * For each person, we return the number of flowers in full bloom during their arrival.
 *
 *  
 * Constraints:
 *
 *     1 <= flowers.length <= 5 * 10^4
 *     flowers[i].length == 2
 *     1 <= starti <= endi <= 10^9
 *     1 <= people.length <= 5 * 10^4
 *     1 <= people[i] <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn full_bloom_flowers(mut flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
		let mut people: Vec<_> = people.into_iter().zip(0..).collect();
		people.sort_unstable();
		flowers.sort_unstable();
		let mut h = 0;
		let mut in_bloom = std::collections::BinaryHeap::new();
		let mut ans = vec![0; people.len()];
		for (p, idx) in people {
			while h < flowers.len() && flowers[h][0] <= p {
				in_bloom.push(-flowers[h][1]);
				h += 1;
			}
			while !in_bloom.is_empty() && in_bloom.peek() > Some(&-p) {
				in_bloom.pop();
			}
			ans[idx] = in_bloom.len() as i32;
		}
		ans
	}
}

// submission codes end
