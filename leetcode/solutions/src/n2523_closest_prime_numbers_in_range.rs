/**
 * [2523] Closest Prime Numbers in Range
 *
 * Given two positive integers left and right, find the two integers num1 and num2 such that:
 *
 *     left <= nums1 < nums2 <= right .
 *     nums1 and nums2 are both prime numbers.
 *     nums2 - nums1 is the minimum amongst all other pairs satisfying the above conditions.
 *
 * Return the positive integer array ans = [nums1, nums2]. If there are multiple pairs satisfying these conditions, return the one with the minimum nums1 value or [-1, -1] if such numbers do not exist.
 * A number greater than 1 is called prime if it is only divisible by 1 and itself.
 *  
 * <strong class="example">Example 1:
 *
 * Input: left = 10, right = 19
 * Output: [11,13]
 * Explanation: The prime numbers between 10 and 19 are 11, 13, 17, and 19.
 * The closest gap between any pair is 2, which can be achieved by [11,13] or [17,19].
 * Since 11 is smaller than 17, we return the first pair.
 *
 * <strong class="example">Example 2:
 *
 * Input: left = 4, right = 6
 * Output: [-1,-1]
 * Explanation: There exists only one prime number in the given range, so the conditions cannot be satisfied.
 *
 *  
 * Constraints:
 *
 *     1 <= left <= right <= 10^6
 *
 *  
 * <style type="text/css">.spoilerbutton {display:block; border:dashed; padding: 0px 0px; margin:10px 0px; font-size:150%; font-weight: bold; color:#000000; background-color:cyan; outline:0;
 * }
 * .spoiler {overflow:hidden;}
 * .spoiler > div {-webkit-transition: all 0s ease;-moz-transition: margin 0s ease;-o-transition: all 0s ease;transition: margin 0s ease;}
 * .spoilerbutton[value="Show Message"] + .spoiler > div {margin-top:-500%;}
 * .spoilerbutton[value="Hide Message"] + .spoiler {padding:5px;}
 * </style>
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
		let mut set = vec![];
		'out: for i in 2..=(right as f32).sqrt() as i32 {
			let lim = (i as f32).sqrt() as i32;
			for &j in &set {
				if j > lim {
					break;
				}
				if i % j == 0 {
					continue 'out;
				}
			}
			set.push(i);
		}
		let mut prv = None;
		let mut ans = (-1, -1);
		let mut min_dif = right;
		'out: for i in left.max(2)..=right {
			let lim = (i as f32).sqrt() as i32;
			for &j in &set {
				if j > lim {
					break;
				}
				if i % j == 0 {
					continue 'out;
				}
			}

			if let Some(prv) = prv {
				if i - prv < min_dif {
					ans = (prv, i);
					min_dif = i - prv;
				}
			}
			prv = Some(i);
		}

		vec![ans.0, ans.1]
	}
}

// submission codes end

#[cfg(test)]
mod tests {

	#[test]
	fn test_2523() {}
}
