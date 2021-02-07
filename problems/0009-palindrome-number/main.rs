impl Solution {
    pub fn is_palindrome(x32: i32) -> bool {
        if x32 < 0 {
            return false;
        }
        
        let x = x32 as i64;
        
        let mut mut_x = x;
        let mut rev_x = 0;
        
        while mut_x > 0 {
            let a = mut_x % 10;
            mut_x /= 10;
            rev_x = rev_x * 10 + a;
        }
        
        return rev_x == x;
    }
}
