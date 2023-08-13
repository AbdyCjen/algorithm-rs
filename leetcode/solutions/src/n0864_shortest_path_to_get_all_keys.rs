/**
 * [864] Shortest Path to Get All Keys
 *
 * You are given an m x n grid grid where:
 *
 *     '.' is an empty cell.
 *     '#' is a wall.
 *     '@' is the starting point.
 *     Lowercase letters represent keys.
 *     Uppercase letters represent locks.
 *
 * You start at the starting point and one move consists of walking one space in one of the four cardinal directions. You cannot walk outside the grid, or walk into a wall.
 * If you walk over a key, you can pick it up and you cannot walk over a lock unless you have its corresponding key.
 * For some <font face="monospace">1 <= k <= 6</font>, there is exactly one lowercase and one uppercase letter of the first k letters of the English alphabet in the grid. This means that there is exactly one key for each lock, and one lock for each key; and also that the letters used to represent the keys and locks were chosen in the same order as the English alphabet.
 * Return the lowest number of moves to acquire all keys. If it is impossible, return -1.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-keys2.jpg" style="width: 404px; height: 245px;" />
 * Input: grid = ["@.a..","###.#","b.A.B"]
 * Output: 8
 * Explanation: Note that the goal is to obtain all the keys not to open all the locks.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-key2.jpg" style="width: 404px; height: 245px;" />
 * Input: grid = ["@..aA","..B#.","....b"]
 * Output: 6
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-keys3.jpg" style="width: 244px; height: 85px;" />
 * Input: grid = ["@Aa"]
 * Output: -1
 *
 *  
 * Constraints:
 *
 *     m == grid.length
 *     n == grid[i].length
 *     1 <= m, n <= 30
 *     grid[i][j] is either an English letter, '.', '#', or '@'.
 *     The number of keys in the grid is in the range [1, 6].
 *     Each key in the grid is unique.
 *     Each key in the grid has a matching lock.
 *
 */
pub struct Solution {}

// submission codes start here

//TODO: neater
impl Solution {
	pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
		let (m, n) = (grid.len(), grid[0].len());
		let mut st = vec![];
		let mut total_keys = 0;
		for (i, row) in (0..).zip(&grid) {
			for (j, c) in (0..).zip(row.bytes()) {
				match c {
					b'@' => st.push((i, j, 0)),
					c @ b'a'..=b'z' => total_keys |= 1 << (c - b'a') as i32,
					_ => {}
				}
			}
		}

		let mut visited = vec![vec![vec![0; 0]; n]; m];
		let (m, n, mut ans) = (m as i32, n as i32, 0);
		while !st.is_empty() {
			for (i, j, mut keys) in std::mem::take(&mut st) {
				match grid[i as usize].as_bytes()[j as usize] {
					c @ b'A'..=b'Z' => {
						let lock = 1 << (c - b'A') as i32;
						if lock & keys == 0 {
							continue;
						}
					}
					c @ b'a'..=b'z' => keys |= 1 << (c - b'a') as i32,
					b'#' => continue,
					_ => {}
				}
				if keys == total_keys {
					return ans;
				}
				visited[i as usize][j as usize].push(keys);
				'dir: for (ni, nj) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
					if ni >= 0 && nj >= 0 && ni < m && nj < n {
						let mut inserted = false;
						for vkeys in &mut visited[ni as usize][nj as usize] {
							let ckeys = *vkeys & keys;
							if ckeys == keys {
								continue 'dir;
							} else if ckeys == *vkeys {
								*vkeys = keys;
								inserted = true;
								break;
							}
						}
						if !inserted {
							visited[ni as usize][nj as usize].push(keys);
						}
						st.push((ni, nj, keys));
					}
				}
			}
			ans += 1;
		}
		-1
	}
}

// submission codes end
