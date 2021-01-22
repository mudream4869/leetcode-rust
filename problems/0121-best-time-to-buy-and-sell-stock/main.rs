impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut mm = 0;
        let mut ans = 0;
        for i in (0..n).rev() {
            if prices[i] > mm {
                mm = prices[i];
            }
            let v = mm - prices[i];
            if ans < v {
                ans = v;
            }
        }
        ans
    }
}
