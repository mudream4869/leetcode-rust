impl Solution {
    pub fn judge_square_sum(mut c: i32) -> bool {
        if c == 0 || c == 1 {
            return true;
        }
        
        while c % 2 == 0 {
            c /= 2;
        }

        let mut i = 3;
        while i*i <= c {
            if c%i == 0 {
                let mut n = 0;
                // i should be prime
                
                while c%i == 0 {
                    c /= i;
                    n += 1;
                }
                
                if n%2 == 1 && i%4 == 3 {
                    return false;
                }
            }
            i += 2;
        }
        
        return c%4 == 1;
    }
}
