/**
 * [1926] Nearest Exit from Entrance in Maze
 *
 * You are given an m x n matrix maze (0-indexed) with empty cells (represented as '.') and walls (represented as '+'). You are also given the entrance of the maze, where entrance = [entrancerow, entrancecol] denotes the row and column of the cell you are initially standing at.
 * In one step, you can move one cell up, down, left, or right. You cannot step into a cell with a wall, and you cannot step outside the maze. Your goal is to find the nearest exit from the entrance. An exit is defined as an empty cell that is at the border of the maze. The entrance does not count as an exit.
 * Return the number of steps in the shortest path from the entrance to the nearest exit, or -1 if no such path exists.
 *  
 * <strong class='example'>Example 1:
 * <img alt='' src='https://assets.leetcode.com/uploads/2021/06/04/nearest1-grid.jpg' style='width: 333px; height: 253px;' />
 * Input: maze = [['+','+','.','+'],['.','.','.','+'],['+','+','+','.']], entrance = [1,2]
 * Output: 1
 * Explanation: There are 3 exits in this maze at [1,0], [0,2], and [2,3].
 * Initially, you are at the entrance cell [1,2].
 * - You can reach [1,0] by moving 2 steps left.
 * - You can reach [0,2] by moving 1 step up.
 * It is impossible to reach [2,3] from the entrance.
 * Thus, the nearest exit is [0,2], which is 1 step away.
 *
 * <strong class='example'>Example 2:
 * <img alt='' src='https://assets.leetcode.com/uploads/2021/06/04/nearesr2-grid.jpg' style='width: 253px; height: 253px;' />
 * Input: maze = [['+','+','+'],['.','.','.'],['+','+','+']], entrance = [1,0]
 * Output: 2
 * Explanation: There is 1 exit in this maze at [1,2].
 * [1,0] does not count as an exit since it is the entrance cell.
 * Initially, you are at the entrance cell [1,0].
 * - You can reach [1,2] by moving 2 steps right.
 * Thus, the nearest exit is [1,2], which is 2 steps away.
 *
 * <strong class='example'>Example 3:
 * <img alt='' src='https://assets.leetcode.com/uploads/2021/06/04/nearest3-grid.jpg' style='width: 173px; height: 93px;' />
 * Input: maze = [['.','+']], entrance = [0,0]
 * Output: -1
 * Explanation: There are no exits in this maze.
 *
 *  
 * Constraints:
 *
 *     maze.length == m
 *     maze[i].length == n
 *     1 <= m, n <= 100
 *     maze[i][j] is either '.' or '+'.
 *     entrance.length == 2
 *     0 <= entrancerow < m
 *     0 <= entrancecol < n
 *     entrance will always be an empty cell.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn nearest_exit(mut maze: Vec<Vec<char>>, ent: Vec<i32>) -> i32 {
		let mut dq = vec![(ent[0], ent[1])];
		let (m, n) = (maze.len() as i32, maze[0].len() as i32);

		let mut i = 0;
		maze[ent[0] as usize][ent[1] as usize] = '+';
		while !dq.is_empty() {
			for cur in dq.split_off(0) {
				if i > 0 && (cur.0 == 0 || cur.0 == m - 1 || cur.1 == 0 || cur.1 == n - 1) {
					return i;
				}

				for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
					let next = (cur.0 + dir.0, cur.1 + dir.1);
					if (0..m).contains(&next.0)
						&& (0..n).contains(&next.1)
						&& maze[next.0 as usize][next.1 as usize] == '.'
					{
						maze[next.0 as usize][next.1 as usize] = '+';
						dq.push(next);
					}
				}
			}
			i += 1;
		}
		-1
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1926() {
		assert_eq!(
			Solution::nearest_exit(
				matrix![['+', '+', '+'], ['.', '.', '.'], ['+', '+', '+']],
				vec![1, 0]
			),
			2
		);
		assert_eq!(
			Solution::nearest_exit(
				matrix![
					['+', '+', '.', '+'],
					['.', '.', '.', '+'],
					['+', '+', '+', '.']
				],
				vec![1, 2]
			),
			1
		);
		assert_eq!(
			Solution::nearest_exit(
				matrix![['+', '+', '+'], ['.', '.', '.'], ['+', '+', '+']],
				vec![1, 0]
			),
			2
		);
	}
}
