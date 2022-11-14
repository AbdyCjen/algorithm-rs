/**
 * [901] Online Stock Span
 *
 * Design an algorithm that collects daily price quotes for some stock and returns the span of that stock's price for the current day.
 * The span of the stock's price today is defined as the maximum number of consecutive days (starting from today and going backward) for which the stock price was less than or equal to today's price.
 *
 *     For example, if the price of a stock over the next 7 days were [100,80,60,70,60,75,85], then the stock spans would be [1,1,1,2,1,4,6].
 *
 * Implement the StockSpanner class:
 *
 *     StockSpanner() Initializes the object of the class.
 *     int next(int price) Returns the span of the stock's price given that today's price is price.
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input
 * ["StockSpanner", "next", "next", "next", "next", "next", "next", "next"]
 * [[], [100], [80], [60], [70], [60], [75], [85]]
 * Output
 * [null, 1, 1, 1, 2, 1, 4, 6]
 * Explanation
 * StockSpanner stockSpanner = new StockSpanner();
 * stockSpanner.next(100); // return 1
 * stockSpanner.next(80);  // return 1
 * stockSpanner.next(60);  // return 1
 * stockSpanner.next(70);  // return 2
 * stockSpanner.next(60);  // return 1
 * stockSpanner.next(75);  // return 4, because the last 4 prices (including today's price of 75) were less than or equal to today's price.
 * stockSpanner.next(85);  // return 6
 *
 *  
 * Constraints:
 *
 *     1 <= price <= 10^5
 *     At most 10^4 calls will be made to next.
 *
 */
pub struct Solution {}

// submission codes start here

struct StockSpanner {
	st: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
	fn new() -> Self { Self { st: Vec::new() } }

	fn next(&mut self, price: i32) -> i32 {
		let mut cnt = 1;
		while let Some(&(n, i)) = self.st.last() {
			if n <= price {
				cnt += i;
				self.st.pop();
			} else {
				break;
			}
		}
		self.st.push((price, cnt));
		cnt
	}
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_901() {}
}
