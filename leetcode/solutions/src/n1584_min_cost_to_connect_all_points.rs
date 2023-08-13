/**
 * [1584] Min Cost to Connect All Points
 *
 * You are given an array points representing integer coordinates of some points on a 2D-plane, where points[i] = [xi, yi].
 * The cost of connecting two points [xi, yi] and [xj, yj] is the manhattan distance between them: |xi - xj| + |yi - yj|, where |val| denotes the absolute value of val.
 * Return the minimum cost to make all points connected. All points are connected if there is exactly one simple path between any two points.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/26/d.png" style="width: 214px; height: 268px;" />
 * Input: points = [[0,0],[2,2],[3,10],[5,2],[7,0]]
 * Output: 20
 * Explanation:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/26/c.png" style="width: 214px; height: 268px;" />
 * We can connect the points as shown above to get the minimum cost of 20.
 * Notice that there is a unique path between every pair of points.
 *
 * <strong class="example">Example 2:
 *
 * Input: points = [[3,12],[-2,5],[-4,1]]
 * Output: 18
 *
 *  
 * Constraints:
 *
 *     1 <= points.length <= 1000
 *     -10^6 <= xi, yi <= 10^6
 *     All pairs (xi, yi) are distinct.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
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
		let mut edges = std::collections::BinaryHeap::new();
		for (i, a) in (0..).zip(&points) {
			for (j, b) in (i + 1..).zip(&points[i as usize + 1..]) {
				let weight = (a[0] - b[0]).abs() + (b[1] - a[1]).abs();
				edges.push((-weight, i, j));
			}
		}
		let (mut cnt, mut ans) = (points.len(), 0);
		let mut set: Vec<i32> = (0..cnt as i32).collect();
		while cnt > 1 {
			let (wei, a, b) = edges.pop().unwrap();
			if find(&mut set, a) != find(&mut set, b) {
				cnt -= 1;
				union(&mut set, a, b);
				ans -= wei;
			}
		}
		ans
	}
}

// submission codes end
