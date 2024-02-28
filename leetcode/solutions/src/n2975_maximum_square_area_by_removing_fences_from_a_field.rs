/**
 * [2975] Maximum Square Area by Removing Fences From a Field
 *
 * There is a large (m - 1) x (n - 1) rectangular field with corners at (1, 1) and (m, n) containing some horizontal and vertical fences given in arrays hFences and vFences respectively.
 * Horizontal fences are from the coordinates (hFences[i], 1) to (hFences[i], n) and vertical fences are from the coordinates (1, vFences[i]) to (m, vFences[i]).
 * Return the maximum area of a square field that can be formed by removing some fences (possibly none) or -1 if it is impossible to make a square field.
 * Since the answer may be large, return it modulo 10^9 + 7.
 * Note: The field is surrounded by two horizontal fences from the coordinates (1, 1) to (1, n) and (m, 1) to (m, n) and two vertical fences from the coordinates (1, 1) to (m, 1) and (1, n) to (m, n). These fences cannot be removed.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/11/05/screenshot-from-2023-11-05-22-40-25.png" />
 *
 * Input: m = 4, n = 3, hFences = [2,3], vFences = [2]
 * Output: 4
 * Explanation: Removing the horizontal fence at 2 and the vertical fence at 2 will give a square field of area 4.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/11/22/maxsquareareaexample1.png" style="width: 285px; height: 242px;" />
 *
 * Input: m = 6, n = 7, hFences = [2], vFences = [4]
 * Output: -1
 * Explanation: It can be proved that there is no way to create a square field by removing fences.
 *
 *  
 * Constraints:
 *
 *     3 <= m, n <= 10^9
 *     <font face="monospace">1 <= hF</font>ences<font face="monospace">.length, vFences.length <= 600</font>
 *     <font face="monospace">1 < hFences[i] < m</font>
 *     <font face="monospace">1 < vFences[i] < n</font>
 *     <font face="monospace">hFences</font><font face="monospace"> and </font><font face="monospace">vFences</font><font face="monospace"> are unique.</font>
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn maximize_square_area(m: i32, n: i32, mut hor: Vec<i32>, mut ver: Vec<i32>) -> i32 {
		let mut cnts = std::collections::HashSet::<i32>::new();
		ver.extend([1, n]);
		ver.sort_unstable();
		for (i, &v) in (0..).zip(&ver) {
			for &prv in &ver[..i] {
				cnts.insert(v - prv);
			}
		}
		hor.extend([1, m]);
		hor.sort_unstable();
		let mut ans = 0;
		for (i, &h) in (0..).zip(&hor) {
			for &prv in &hor[..i] {
				if cnts.contains(&(h - prv)) {
					ans = ans.max(h - prv);
				}
			}
		}
		match ans {
			0 => -1,
			a => ((a as i64 * a as i64) % (1e9 as i64 + 7)) as i32,
		}
	}
}

// submission codes end
