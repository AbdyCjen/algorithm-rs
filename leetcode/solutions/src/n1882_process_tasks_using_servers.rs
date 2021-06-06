pub struct Solution {}
#[allow(dead_code)]
impl Solution {
	pub fn assign_tasks(servers: Vec<i32>, mut tasks: Vec<i32>) -> Vec<i32> {
		use std::{
			cmp::Reverse,
			collections::{BTreeMap, BinaryHeap},
		};
		let mut avails: BinaryHeap<_> = servers
			.into_iter()
			.enumerate()
			.map(|(i, j)| Reverse((j, i as i32)))
			.collect();
		let n = tasks.len() as i32;
		let mut working: BTreeMap<_, Vec<_>> = BTreeMap::new();
		let mut ans = Vec::new();
		tasks.reverse();

		let mut i = 0;
		while !tasks.is_empty() {
			avails.extend(working.remove(&i).into_iter().flatten());
			if avails.is_empty() {
				i = *working.iter().next().unwrap().0;
				continue;
			}

			while !avails.is_empty() && !tasks.is_empty() && i + tasks.len() as i32 != n {
				if let Some(Reverse(ass_svr)) = avails.pop() {
					working
						.entry(i + tasks.pop().unwrap())
						.or_default()
						.push(Reverse(ass_svr));
					ans.push(ass_svr.1);
				}
			}
			i += 1;
		}
		ans
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1882() {
		assert_eq!(
			Solution::assign_tasks(vec![3, 3, 2], vec![1, 2, 3, 2, 1, 2]),
			vec![2, 2, 0, 2, 1, 2]
		);
		assert_eq!(
			Solution::assign_tasks(vec![5, 1, 4, 3, 2], vec![2, 1, 2, 4, 5, 2, 1]),
			vec![1, 4, 1, 4, 1, 3, 2]
		);
	}
}
