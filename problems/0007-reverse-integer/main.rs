impl Solution {
    pub fn reverse(px: i32) -> i32 {
        let mut x: i64 = px as i64;
        let is_neg = (x <= 0);
        if is_neg {
            x = -x;
        }
        
        let mut ans = 0;
        while x > 0 {
            let a = x%10;
            x /= 10;            
            ans = ans*10 + a;
        }
        
        if is_neg {
            ans = -ans;
        }
        
        if ans > std::i32::MAX as i64 {
            return 0;
        } else if ans < std::i32::MIN as i64 {
            return 0;
        }
        
        ans as i32
    }
}
