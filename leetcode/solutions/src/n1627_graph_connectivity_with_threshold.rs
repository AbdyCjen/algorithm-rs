/**
 * [1627] Graph Connectivity With Threshold
 *
 * We have n cities labeled from 1 to n. Two different cities with labels x and y are directly connected by a bidirectional road if and only if x and y share a common divisor strictly greater than some threshold. More formally, cities with labels x and y have a road between them if there exists an integer z such that all of the following are true:
 *
 *     x % z == 0,
 *     y % z == 0, and
 *     z > threshold.
 *
 * Given the two integers, n and threshold, and an array of queries, you must determine for each queries[i] = [ai, bi] if cities ai and bi are connected directly or indirectly. (i.e. there is some path between them).
 * Return an array answer, where answer.length == queries.length and answer[i] is true if for the i^th query, there is a path between ai and bi, or answer[i] is false if there is no path.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/09/ex1.jpg" style="width: 382px; height: 181px;" />
 * Input: n = 6, threshold = 2, queries = [[1,4],[2,5],[3,6]]
 * Output: [false,false,true]
 * Explanation: The divisors for each number:
 * 1:   1
 * 2:   1, 2
 * 3:   1, <u>3</u>
 * 4:   1, 2, <u>4</u>
 * 5:   1, <u>5</u>
 * 6:   1, 2, <u>3</u>, <u>6</u>
 * Using the underlined divisors above the threshold, only cities 3 and 6 share a common divisor, so they are the
 * only ones directly connected. The result of each query:
 * [1,4]   1 is not connected to 4
 * [2,5]   2 is not connected to 5
 * [3,6]   3 is connected to 6 through path 3--6
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/10/tmp.jpg" style="width: 532px; height: 302px;" />
 * Input: n = 6, threshold = 0, queries = [[4,5],[3,4],[3,2],[2,6],[1,3]]
 * Output: [true,true,true,true,true]
 * Explanation: The divisors for each number are the same as the previous example. However, since the threshold is 0,
 * all divisors can be used. Since all numbers share 1 as a divisor, all cities are connected.
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/17/ex3.jpg" style="width: 282px; height: 282px;" />
 * Input: n = 5, threshold = 1, queries = [[4,5],[4,5],[3,2],[2,3],[3,4]]
 * Output: [false,false,false,false,false]
 * Explanation: Only cities 2 and 4 share a common divisor 2 which is strictly greater than the threshold 1, so they are the only ones directly connected.
 * Please notice that there can be multiple queries for the same pair of nodes [x, y], and that the query [x, y] is equivalent to the query [y, x].
 *
 *  
 * Constraints:
 *
 *     2 <= n <= 10^4
 *     0 <= threshold <= n
 *     1 <= queries.length <= 10^5
 *     queries[i].length == 2
 *     1 <= ai, bi <= cities
 *     ai != bi
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
		fn find(set: &mut [i32], x: i32) -> i32 {
			if set[x as usize] != x {
				set[x as usize] = find(set, set[x as usize]);
			}
			set[x as usize]
		}
		fn union(set: &mut [i32], a: i32, b: i32) {
			let a = find(set, a);
			set[a as usize] = find(set, b);
		}
		if threshold == 0 {
			return vec![true; queries.len()];
		}
		let mut set: Vec<_> = (0..=n).collect();

		for i in 0..=n {
			for pr in 1..=(i as f64).sqrt() as i32 {
				if i % pr == 0 {
					if pr > threshold {
						union(&mut set, i, pr);
					}
					if i / pr > threshold {
						union(&mut set, i, i / pr);
					}
				}
			}
		}
		queries
			.into_iter()
			.map(|q| find(&mut set, q[0]) == find(&mut set, q[1]))
			.collect()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1627() {
		assert_eq!(
			Solution::are_connected(6, 0, matrix![[4, 5], [3, 4], [3, 2], [2, 6], [1, 3]]),
			vec![true; 5]
		);
		assert_eq!(Solution::are_connected(6, 2, matrix![[6, 3]]), [true]);
		assert_eq!(Solution::are_connected(26, 3, matrix![[8, 3]]), [false]);
	}
}
