/**
 * [904] Fruit Into Baskets
 *
 * You are visiting a farm that has a single row of fruit trees arranged from left to right. The trees are represented by an integer array fruits where fruits[i] is the type of fruit the i^th tree produces.
 * You want to collect as much fruit as possible. However, the owner has some strict rules that you must follow:
 *
 *     You only have two baskets, and each basket can only hold a single type of fruit. There is no limit on the amount of fruit each basket can hold.
 *     Starting from any tree of your choice, you must pick exactly one fruit from every tree (including the start tree) while moving to the right. The picked fruits must fit in one of your baskets.
 *     Once you reach a tree with fruit that cannot fit in your baskets, you must stop.
 *
 * Given the integer array fruits, return the maximum number of fruits you can pick.
 *  
 * <strong class="example">Example 1:
 *
 * Input: fruits = [<u>1,2,1</u>]
 * Output: 3
 * Explanation: We can pick from all 3 trees.
 *
 * <strong class="example">Example 2:
 *
 * Input: fruits = [0,<u>1,2,2</u>]
 * Output: 3
 * Explanation: We can pick from trees [1,2,2].
 * If we had started at the first tree, we would only pick from trees [0,1].
 *
 * <strong class="example">Example 3:
 *
 * Input: fruits = [1,<u>2,3,2,2</u>]
 * Output: 4
 * Explanation: We can pick from trees [2,3,2,2].
 * If we had started at the first tree, we would only pick from trees [1,2].
 *
 *  
 * Constraints:
 *
 *     1 <= fruits.length <= 10^5
 *     0 <= fruits[i] < fruits.length
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn total_fruit(fruits: Vec<i32>) -> i32 {
		let mut cnt = std::collections::HashMap::new();
		let mut j = 0;
		for &f in &fruits {
			j += 1;
			*cnt.entry(f).or_insert(0) += 1;
			if f != fruits[0] {
				break;
			}
		}
		let mut i = 0;
		let mut ans = j - i as i32;
		for j in j..fruits.len() as i32 {
			let e = cnt.entry(fruits[j as usize]).or_insert(0);
			*e += 1;
			if *e == 1 {
				// new fruit
				loop {
					let e = cnt.get_mut(&fruits[i]).unwrap();
					*e -= 1;
					i += 1;
					if *e == 0 {
						break;
					}
				}
			}
			ans = ans.max(j + 1 - i as i32)
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_904() {
		assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2,]), 3);
		assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
	}
}
