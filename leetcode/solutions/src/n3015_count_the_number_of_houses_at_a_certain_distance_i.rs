/**
 * [3015] Count the Number of Houses at a Certain Distance I
 *
 * You are given three positive integers n, x, and y.
 * In a city, there exist houses numbered 1 to n connected by n streets. There is a street connecting the house numbered i with the house numbered i + 1 for all 1 <= i <= n - 1 . An additional street connects the house numbered x with the house numbered y.
 * For each k, such that 1 <= k <= n, you need to find the number of pairs of houses (house1, house2) such that the minimum number of streets that need to be traveled to reach house2 from house1 is k.
 * Return a 1-indexed array result of length n where result[k] represents the total number of pairs of houses such that the minimum streets required to reach one house from the other is k.
 * Note that x and y can be equal.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/12/20/example2.png" style="width: 474px; height: 197px;" />
 * Input: n = 3, x = 1, y = 3
 * Output: [6,0,0]
 * Explanation: Let's look at each pair of houses:
 * - For the pair (1, 2), we can go from house 1 to house 2 directly.
 * - For the pair (2, 1), we can go from house 2 to house 1 directly.
 * - For the pair (1, 3), we can go from house 1 to house 3 directly.
 * - For the pair (3, 1), we can go from house 3 to house 1 directly.
 * - For the pair (2, 3), we can go from house 2 to house 3 directly.
 * - For the pair (3, 2), we can go from house 3 to house 2 directly.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/12/20/example3.png" style="width: 668px; height: 174px;" />
 * Input: n = 5, x = 2, y = 4
 * Output: [10,8,2,0,0]
 * Explanation: For each distance k the pairs are:
 * - For k == 1, the pairs are (1, 2), (2, 1), (2, 3), (3, 2), (2, 4), (4, 2), (3, 4), (4, 3), (4, 5), and (5, 4).
 * - For k == 2, the pairs are (1, 3), (3, 1), (1, 4), (4, 1), (2, 5), (5, 2), (3, 5), and (5, 3).
 * - For k == 3, the pairs are (1, 5), and (5, 1).
 * - For k == 4 and k == 5, there are no pairs.
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/12/20/example5.png" style="width: 544px; height: 130px;" />
 * Input: n = 4, x = 1, y = 1
 * Output: [6,4,2,0]
 * Explanation: For each distance k the pairs are:
 * - For k == 1, the pairs are (1, 2), (2, 1), (2, 3), (3, 2), (3, 4), and (4, 3).
 * - For k == 2, the pairs are (1, 3), (3, 1), (2, 4), and (4, 2).
 * - For k == 3, the pairs are (1, 4), and (4, 1).
 * - For k == 4, there are no pairs.
 *
 *  
 * Constraints:
 *
 *     2 <= n <= 100
 *     1 <= x, y <= n
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i32> {
		let mut graph = vec![vec![i32::MAX; n as usize]; n as usize];
		for i in 0..n as usize - 1 {
			graph[i][i + 1] = 1;
			graph[i + 1][i] = 1;
		}
		graph[x as usize - 1][y as usize - 1] = 1;
		graph[y as usize - 1][x as usize - 1] = 1;
		Self::floyd(&mut graph);
		let mut ans = vec![0; n as usize];
		for (i, row) in (0..).zip(&graph) {
			for (j, &n) in (0..).zip(row) {
				if i != j {
					ans[n as usize - 1] += 1;
				}
			}
		}
		ans
	}
	fn floyd(graph: &mut [Vec<i32>]) {
		let n = graph.len();
		for k in 0..n {
			for x in 0..n {
				for y in 0..n {
					graph[x][y] = graph[x][y].min(graph[x][k].saturating_add(graph[k][y]));
				}
			}
		}
	}
}

// submission codes end
