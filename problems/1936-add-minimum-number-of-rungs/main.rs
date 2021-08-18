impl Solution {
    pub fn add_rungs(rungs: Vec<i32>, dist: i32) -> i32 {
        let mut last_h = 0;
        let mut ans = 0;
        for v in rungs {
            ans += (v - last_h - 1)/dist;
            last_h = v;
        }
        ans
    }
}
