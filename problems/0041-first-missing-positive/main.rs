impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        // To make nums to be a f: i -> i + 1
        let n = nums.len() as i32;
        for i in 0..nums.len() {
            let mut procn = nums[i];
            loop {
                if procn < 1  || procn > n {
                    break;
                }
                if nums[(procn-1) as usize] == procn {
                    break;
                }
                let tmp = nums[(procn - 1) as usize];
                nums[(procn - 1) as usize] = procn;
                procn = tmp;
            }
        }
        
        for i in 0..nums.len() {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        return n + 1;
    }
}
