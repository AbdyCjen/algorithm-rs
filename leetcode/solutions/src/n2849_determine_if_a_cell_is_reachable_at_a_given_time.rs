/**
 * [2849] Determine if a Cell Is Reachable at a Given Time
 *
 * You are given four integers sx, sy, fx, fy, and a non-negative integer t.
 * In an infinite 2D grid, you start at the cell (sx, sy). Each second, you must move to any of its adjacent cells.
 * Return true if you can reach cell (fx, fy) after exactly t seconds, or false otherwise.
 * A cell's adjacent cells are the 8 cells around it that share at least one corner with it. You can visit the same cell several times.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/08/05/example2.svg" style="width: 443px; height: 243px;" />
 * Input: sx = 2, sy = 4, fx = 7, fy = 7, t = 6
 * Output: true
 * Explanation: Starting at cell (2, 4), we can reach cell (7, 7) in exactly 6 seconds by going through the cells depicted in the picture above.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/08/05/example1.svg" style="width: 383px; height: 202px;" />
 * Input: sx = 3, sy = 1, fx = 7, fy = 3, t = 3
 * Output: false
 * Explanation: Starting at cell (3, 1), it takes at least 4 seconds to reach cell (7, 3) by going through the cells depicted in the picture above. Hence, we cannot reach cell (7, 3) at the third second.
 *
 *  
 * Constraints:
 *
 *     1 <= sx, sy, fx, fy <= 10^9
 *     0 <= t <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
		if fx == sx && sy == fy {
			t != 1
		} else {
			(sx - fx).abs().max((sy - fy).abs()) <= t
		}
	}
}

// submission codes end
