/**
 * [1254] Number of Closed Islands
 *
 * Given a 2D grid consists of 0s (land) and 1s (water).  An island is a maximal 4-directionally connected group of <font face="monospace">0</font>s and a closed island is an island totally (all left, top, right, bottom) surrounded by 1s.
 * Return the number of closed islands.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/31/sample_3_1610.png" style="width: 240px; height: 120px;" />
 *
 * Input: grid = [[1,1,1,1,1,1,1,0],[1,0,0,0,0,1,1,0],[1,0,1,0,1,1,1,0],[1,0,0,0,0,1,0,1],[1,1,1,1,1,1,1,0]]
 * Output: 2
 * Explanation:
 * Islands in gray are closed because they are completely surrounded by water (group of 1s).
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/31/sample_4_1610.png" style="width: 160px; height: 80px;" />
 *
 * Input: grid = [[0,0,1,0,0],[0,1,0,1,0],[0,1,1,1,0]]
 * Output: 1
 *
 * <strong class="example">Example 3:
 *
 * Input: grid = [[1,1,1,1,1,1,1],
 *                [1,0,0,0,0,0,1],
 *                [1,0,1,1,1,0,1],
 *                [1,0,1,0,1,0,1],
 *                [1,0,1,1,1,0,1],
 *                [1,0,0,0,0,0,1],
 *                [1,1,1,1,1,1,1]]
 * Output: 2
 *
 *  
 * Constraints:
 *
 *     1 <= grid.length, grid[0].length <= 100
 *     0 <= grid[i][j] <=1
 *
 */
pub struct Solution {}

impl Solution {
	fn dfs(grid: &mut [Vec<i32>], i: usize, j: usize) {
		grid[i][j] = 1;
		if i > 0 && grid[i - 1][j] == 0 {
			Self::dfs(grid, i - 1, j)
		};
		if i + 1 < grid.len() && grid[i + 1][j] == 0 {
			Self::dfs(grid, i + 1, j)
		};
		if j > 0 && grid[i][j - 1] == 0 {
			Self::dfs(grid, i, j - 1)
		};
		if j + 1 < grid[0].len() && grid[i][j + 1] == 0 {
			Self::dfs(grid, i, j + 1)
		};
	}
	pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
		let (m, n) = (grid.len(), grid[0].len());
		for j in 0..n {
			if grid[0][j] == 0 {
				Self::dfs(&mut grid, 0, j);
			}
			if grid[m - 1][j] == 0 {
				Self::dfs(&mut grid, m - 1, j);
			}
		}
		for i in 0..m {
			if grid[i][0] == 0 {
				Self::dfs(&mut grid, i, 0);
			}
			if grid[i][n - 1] == 0 {
				Self::dfs(&mut grid, i, n - 1);
			}
		}

		let mut ans = 0;
		for i in 0..m {
			for j in 0..n {
				if grid[i][j] == 0 {
					ans += 1;
					Self::dfs(&mut grid, i, j);
				}
			}
		}
		ans
	}
	pub fn closed_island_1(grid: Vec<Vec<i32>>) -> i32 {
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
		let (m, n) = (grid.len(), grid[0].len());
		let mut set = (0..(m * n) as i32).collect::<Vec<_>>();
		let mut lands = vec![];
		for (r, i) in grid.iter().zip(0..) {
			for (_, j) in r.iter().zip(0..).filter(|(&b, _)| b == 0) {
				let idx = (i * n + j) as i32;
				lands.push(idx);
				if i > 0 && grid[i - 1][j] == 0 {
					union(&mut set, idx, idx - n as i32)
				}
				if i < m - 1 && grid[i + 1][j] == 0 {
					union(&mut set, idx, idx + n as i32)
				}
				if j < n - 1 && grid[i][j + 1] == 0 {
					union(&mut set, idx, idx + 1)
				}
				if j > 0 && grid[i][j - 1] == 0 {
					union(&mut set, idx, idx - 1)
				}
			}
		}
		let mut cnts = std::collections::HashSet::new();
		for i in lands {
			cnts.insert(find(&mut set, i));
		}
		for i in (0..n).chain(((m - 1) * n)..(m * n)) {
			cnts.remove(&find(&mut set, i as i32));
		}
		for i in (0..m * n).step_by(n).chain((n - 1..m * n).step_by(n)) {
			cnts.remove(&find(&mut set, i as i32));
		}
		cnts.len() as i32
	}
}
