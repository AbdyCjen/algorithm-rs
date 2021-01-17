/**
 * [605] Can Place Flowers
 *
 * You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers cannot be planted in adjacent plots.
 * Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n, return if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule.
 *  
 * Example 1:
 * Input: flowerbed = [1,0,0,0,1], n = 1
 * Output: true
 * Example 2:
 * Input: flowerbed = [1,0,0,0,1], n = 2
 * Output: false
 *  
 * Constraints:
 *
 * 	1 <= flowerbed.length <= 2 * 10^4
 * 	flowerbed[i] is 0 or 1.
 * 	There are no two adjacent flowers in flowerbed.
 * 	0 <= n <= flowerbed.length
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn can_place_flowers(flowerbed: Vec<i32>, mut n: i32) -> bool {
		let mut flowerbed = flowerbed.into_iter().peekable();
		while let (Some(o), _n @ 1..=std::i32::MAX) = (flowerbed.next(), n) {
			match o {
				0 => {
					if 0 == flowerbed.peek().copied().unwrap_or(0) {
						flowerbed.next();
						n -= 1;
					}
				}
				1 => {
					flowerbed.next();
				}
				_ => unreachable!(),
			}
		}
		n == 0
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_605() {
		assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
		assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
		assert_eq!(
			Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2),
			false
		);
		assert_eq!(Solution::can_place_flowers(vec![0, 1, 0], 1), false);
	}
}
