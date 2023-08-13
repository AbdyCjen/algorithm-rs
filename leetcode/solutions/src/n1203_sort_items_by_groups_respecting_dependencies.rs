/**
 * [1203] Sort Items by Groups Respecting Dependencies
 *
 * There are n items each belonging to zero or one of m groups where group[i] is the group that the i-th item belongs to and it's equal to -1 if the i-th item belongs to no group. The items and the groups are zero indexed. A group can have no item belonging to it.
 * Return a sorted list of the items such that:
 *
 *     The items that belong to the same group are next to each other in the sorted list.
 *     There are some relations between these items where beforeItems[i] is a list containing all the items that should come before the i-th item in the sorted array (to the left of the i-th item).
 *
 * Return any solution if there is more than one solution and return an empty list if there is no solution.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/09/11/1359_ex1.png" style="width: 191px; height: 181px;" />
 *
 * Input: n = 8, m = 2, group = [-1,-1,1,0,0,1,0,-1], beforeItems = [[],[6],[5],[6],[3,6],[],[],[]]
 * Output: [6,3,4,1,5,2,0,7]
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 8, m = 2, group = [-1,-1,1,0,0,1,0,-1], beforeItems = [[],[6],[5],[6],[3],[],[4],[]]
 * Output: []
 * Explanation: This is the same as example 1 except that 4 needs to be before 6 in the sorted list.
 *
 *  
 * Constraints:
 *
 *     1 <= m <= n <= 3 * 10^4
 *     group.length == beforeItems.length == n
 *     -1 <= group[i] <= m - 1
 *     0 <= beforeItems[i].length <= n - 1
 *     0 <= beforeItems[i][j] <= n - 1
 *     i != beforeItems[i][j]
 *     beforeItems[i] does not contain duplicates elements.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	fn top_sort(befores: &[Vec<i32>]) -> Vec<i32> {
		let mut cnt = vec![0; befores.len()];
		for &i in befores.iter().flatten() {
			cnt[i as usize] += 1;
		}
		let mut dq = std::collections::VecDeque::new();
		for (i, &c) in (0..).zip(&cnt) {
			if c == 0 {
				dq.push_back(i);
			}
		}
		let mut ans = vec![];
		while let Some(n) = dq.pop_front() {
			ans.push(n);
			for &b in &befores[n as usize] {
				cnt[b as usize] -= 1;
				if cnt[b as usize] == 0 {
					dq.push_back(b);
				}
			}
			cnt[n as usize] -= 1;
		}
		ans.reverse();
		ans
	}
	pub fn sort_items(
		_n: i32,
		m: i32,
		mut group: Vec<i32>,
		before_items: Vec<Vec<i32>>,
	) -> Vec<i32> {
		let mut belongs = vec![vec![]; m as usize];
		for (i, g) in (0..).zip(&mut group) {
			if *g < 0 {
				*g = belongs.len() as i32;
				belongs.push(vec![]);
			}
			belongs[*g as usize].push(i);
		}
		let mut before_groups = vec![vec![]; belongs.len()];
		for (i, befores) in (0..).zip(&before_items) {
			let g = group[i];
			for &be in befores.iter().filter(|be| group[**be as usize] != g) {
				before_groups[group[i] as usize].push(group[be as usize]);
			}
		}

		for g in &mut before_groups {
			g.sort_unstable();
			g.dedup();
		}
		let group_sorted = Self::top_sort(&before_groups);
		let items_sorted = Self::top_sort(&before_items);
		if group_sorted.len() != before_groups.len() || items_sorted.len() != before_items.len() {
			return vec![];
		}
		belongs.iter_mut().for_each(|r| r.clear());
		for it in items_sorted {
			let g = group[it as usize];
			belongs[g as usize].push(it);
		}
		let mut ans = vec![];
		for g in group_sorted {
			ans.extend(belongs[g as usize].iter().copied());
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1203() {
		assert_eq!(
			Solution::sort_items(
				8,
				2,
				vec![-1, -1, 1, 0, 0, 1, 0, -1],
				matrix![[], [6], [5], [6], [3, 6], [], [], []]
			),
			vec![6, 3, 4, 7, 1, 0, 5, 2]
		);
		assert_eq!(
			Solution::sort_items(
				8,
				2,
				vec![-1, -1, 1, 0, 0, 1, 0, -1],
				matrix![[], [6], [5], [6], [3], [], [4], []]
			),
			vec![]
		);
	}
}
