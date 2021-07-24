impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut ptr = (n + m - 1) as usize;
        let mut ptr1 = m - 1;
        let mut ptr2 = n - 1;
        while ptr1 >= 0 || ptr2 >= 0 {
            let mut v1 = i32::MIN;
            let mut v2 = i32::MIN;
            if ptr1 >= 0 {
                v1 = nums1[ptr1 as usize];
            }
            if ptr2 >= 0 {
                v2 = nums2[ptr2 as usize];
            }
            
            if v1 < v2 {
                nums1[ptr] = v2;
                ptr2 -= 1;
            } else {
                nums1[ptr] = v1;
                ptr1 -= 1;
            }
            ptr -= 1;
        }
    }
}
