/**
 * [994] Prison Cells After N Days
 *
 * There are 8 prison cells in a row, and each cell is either occupied or vacant.
 *
 * Each day, whether the cell is occupied or vacant changes according to the following rules:
 *
 *
 * If a cell has two adjacent neighbors that are both occupied or both vacant, then the cell becomes occupied.
 * Otherwise, it becomes vacant.
 *
 *
 * (Note that because the prison is a row, the first and the last cells in the row can't have two adjacent neighbors.)
 *
 * We describe the current state of the prison in the following way: cells[i] == 1 if the i-th cell is occupied, else cells[i] == 0.
 *
 * Given the initial state of the prison, return the state of the prison after N days (and N such changes described above.)
 *
 *  
 *
 * <div>
 * <ol>
 * </ol>
 * </div>
 *
 * <div>
 * Example 1:
 *
 *
 * Input: cells = <span id="example-input-1-1">[0,1,0,1,1,0,0,1]</span>, N = <span id="example-input-1-2">7</span>
 * Output: <span id="example-output-1">[0,0,1,1,0,0,0,0]</span>
 * Explanation:
 * <span id="example-output-1">The following table summarizes the state of the prison on each day:
 * Day 0: [0, 1, 0, 1, 1, 0, 0, 1]
 * Day 1: [0, 1, 1, 0, 0, 0, 0, 0]
 * Day 2: [0, 0, 0, 0, 1, 1, 1, 0]
 * Day 3: [0, 1, 1, 0, 0, 1, 0, 0]
 * Day 4: [0, 0, 0, 0, 0, 1, 0, 0]
 * Day 5: [0, 1, 1, 1, 0, 1, 0, 0]
 * Day 6: [0, 0, 1, 0, 1, 1, 0, 0]
 * Day 7: [0, 0, 1, 1, 0, 0, 0, 0]</span>
 *
 *
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: cells = <span id="example-input-2-1">[1,0,0,1,0,0,1,0]</span>, N = <span id="example-input-2-2">1000000000</span>
 * Output: <span id="example-output-2">[0,0,1,1,1,1,1,0]</span>
 *
 *
 *  
 *
 * Note:
 *
 * <ol>
 * cells.length == 8
 * cells[i] is in {0, 1}
 * 1 <= N <= 10^9
 * </ol>
 * </div>
 * </div>
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
#[allow(dead_code)]
impl Solution {
	pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
		let mut cur_block = 0;
		cells
			.into_iter()
			.enumerate()
			.filter(|(_, c)| *c == 1)
			.for_each(|(i, _)| cur_block |= 1 << i);
		let first_block = cur_block;

		let mut mm = HashMap::<u8, _>::new();
		while !mm.contains_key(&cur_block) {
			let mut next_block = 0;
			for i in 1..7 {
				let bit_ptr = 0b101 << (i - 1);
				if bit_ptr & cur_block == bit_ptr || (bit_ptr ^ cur_block) & bit_ptr == bit_ptr {
					next_block |= 1 << i;
				}
			}
			mm.insert(cur_block, next_block);
			cur_block = next_block;
		}

		cur_block = first_block;
		let mut visited: HashMap<_, i32> = HashMap::new();
		for i in 0..n {
			match visited.get(&cur_block) {
				None => {
					visited.insert(cur_block, i);
					//eprintln!("{:#010b}", cur_block);
					cur_block = mm[&cur_block];
				}
				Some(&prv) => {
					let loop_len = i - prv;
					cur_block = *visited
						.iter()
						.find(|(_, ind)| **ind == (n - prv) % loop_len + prv)
						.unwrap()
						.0;
					break;
				}
			}
		}
		(0..8).map(|i| (cur_block >> i) as i32 & 1).collect()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_957() {
		assert_eq!(
			Solution::prison_after_n_days(vec![1, 0, 0, 1, 0, 0, 1, 0], 1000000000),
			vec![0, 0, 1, 1, 1, 1, 1, 0]
		);
		assert_eq!(
			Solution::prison_after_n_days(vec![0, 1, 0, 1, 1, 0, 0, 1], 7),
			vec![0, 0, 1, 1, 0, 0, 0, 0]
		);
	}
}
