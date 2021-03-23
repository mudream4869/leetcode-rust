impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        
        if nums[0] <= nums[n-1] {
            return nums[0];
        }
        // 5 6 7 1 2 3
        //     ^cut
 
        // [min, max)
        let mut cutMin = 0 as usize;
        let mut cutMax = n as usize;
        
        while (cutMin + 1 < cutMax) {
            let mid = (cutMin + cutMax) >> 1;
            if nums[mid] > nums[0] {
                cutMin = mid;
            } else {
                cutMax = mid;
            }
        }
        nums[cutMin + 1]
    }
}
