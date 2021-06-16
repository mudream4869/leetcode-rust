use std::cmp;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        let chars = s.as_bytes();
        let mut max_len = 1usize;
        let mut ans = (0usize, 1usize);
        
        // 2*k + 1
        for mid in 0..len {
            let mut max_dist = 0usize;
            for dist in 0..(cmp::min(mid, len - 1 - mid) + 1) {
                let ch_left = chars[mid - dist];
                let ch_right = chars[mid + dist];
                if ch_left != ch_right {
                    break;
                }
                max_dist = dist;
            }
            
            if max_dist*2 + 1 > max_len {
                max_len = max_dist*2 + 1;
                ans = (mid - max_dist, mid + max_dist + 1);
            }
        }
        
        // 2*k
        for mid in 0..len {
            let mut max_r = 0usize;
            for r in 1..(cmp::min(mid + 1, len - mid - 1) + 1) {
                let ch_left = chars[mid + 1 - r];
                let ch_right = chars[mid + r];
                if ch_left != ch_right {
                    break;
                }
                max_r = r;
            }
            
            if max_r*2 > max_len {
                max_len = max_r*2;
                ans = (mid - max_r + 1, mid + max_r + 1);
            }
        }
        
        String::from(&s[ans.0..ans.1])
    }
}
