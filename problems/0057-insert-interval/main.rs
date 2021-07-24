use std::cmp;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut proc_seg = new_interval.to_vec();
        let mut ret: Vec<Vec<i32>> = Vec::new();
        for seg in intervals.iter() {
            if proc_seg[0] > seg[1] {
                ret.push(seg.to_vec());
                continue;
            }
            if seg[0] > proc_seg[1] {
                ret.push(proc_seg.to_vec());
                proc_seg[0] = seg[0];
                proc_seg[1] = seg[1];
                continue;
            }
      
            // Merge segments
            proc_seg[0] = cmp::min(proc_seg[0], seg[0]);
            proc_seg[1] = cmp::max(proc_seg[1], seg[1]);
        }
        ret.push(proc_seg);
        ret
    }
}
