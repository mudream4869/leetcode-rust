impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let uk = (k as usize) % nums.len();
        nums.reverse();
        nums[uk..].reverse();
        nums[..uk].reverse();
    }
}
