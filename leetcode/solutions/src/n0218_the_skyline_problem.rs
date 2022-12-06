/**
 * [218] The Skyline Problem
 *
 * A city's skyline is the outer contour of the silhouette formed by all the buildings in that city when viewed from a distance. Given the locations and heights of all the buildings, return the skyline formed by these buildings collectively.
 * The geometric information of each building is given in the array buildings where buildings[i] = [lefti, righti, heighti]:
 *
 *     lefti is the x coordinate of the left edge of the i^th building.
 *     righti is the x coordinate of the right edge of the i^th building.
 *     heighti is the height of the i^th building.
 *
 * You may assume all buildings are perfect rectangles grounded on an absolutely flat surface at height 0.
 * The skyline should be represented as a list of "key points" sorted by their x-coordinate in the form [[x1,y1],[x2,y2],...]. Each key point is the left endpoint of some horizontal segment in the skyline except the last point in the list, which always has a y-coordinate 0 and is used to mark the skyline's termination where the rightmost building ends. Any ground between the leftmost and rightmost buildings should be part of the skyline's contour.
 * Note: There must be no consecutive horizontal lines of equal height in the output skyline. For instance, [...,[2 3],[4 5],[7 5],[11 5],[12 7],...] is not acceptable; the three lines of height 5 should be merged into one in the final output as such: [...,[2 3],[4 5],[12 7],...]
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/01/merged.jpg" style="width: 800px; height: 331px;" />
 * Input: buildings = [[2,9,10],[3,7,15],[5,12,12],[15,20,10],[19,24,8]]
 * Output: [[2,10],[3,15],[7,12],[12,0],[15,10],[20,8],[24,0]]
 * Explanation:
 * Figure A shows the buildings of the input.
 * Figure B shows the skyline formed by those buildings. The red points in figure B represent the key points in the output list.
 *
 * Example 2:
 *
 * Input: buildings = [[0,2,3],[2,5,3]]
 * Output: [[0,3],[5,0]]
 *
 *  
 * Constraints:
 *
 *     1 <= buildings.length <= 10^4
 *     0 <= lefti < righti <= 2^31 - 1
 *     1 <= heighti <= 2^31 - 1
 *     buildings is sorted by lefti in non-decreasing order.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	// 1. the height can only change at every building's left or right
	// 2. iterate over all possible changing point in non-decreasing order
	// 		- record all layers of buildings in the meanwhile
	// 		- and pick the highest layer from it
	pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		let mut heights = Vec::new();
		for n in buildings {
			if let [l, r, h] = n[..] {
				heights.push((l, h));
				heights.push((r, -h));
			}
		}
		heights.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| b.1.cmp(&a.1)));
		let mut cur_hei = std::collections::BTreeMap::new();
		cur_hei.insert(0, 1);
		let mut prev = 0;
		let mut ans = Vec::new();
		for (i, h) in heights {
			if h > 0 {
				*cur_hei.entry(h).or_insert(0) += 1;
			} else if let std::collections::btree_map::Entry::Occupied(mut e) = cur_hei.entry(-h) {
				*e.get_mut() -= 1;
				if *e.get() == 0 {
					cur_hei.remove(&-h);
				}
			}
			if let Some(&cur) = cur_hei.keys().next_back() {
				if cur != prev {
					ans.push(vec![i, cur]);
				}
				prev = cur;
			}
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_218() {
		assert_eq!(
			Solution::get_skyline(matrix![[0, 2, 3], [2, 5, 3]]),
			[[0, 3], [5, 0]]
		);
		assert_eq!(
			Solution::get_skyline(matrix![
				[2, 9, 10],
				[3, 7, 15],
				[5, 12, 12],
				[15, 20, 10],
				[19, 24, 8]
			]),
			[
				[2, 10],
				[3, 15],
				[7, 12],
				[12, 0],
				[15, 10],
				[20, 8],
				[24, 0]
			]
		);
	}
}
