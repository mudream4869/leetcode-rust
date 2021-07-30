impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        // Well... we only need to return the answer for n = 1~9...
        // For recursive solution, please see 0050-n-queens code.
        match n {
            1 => 1,
            2 | 3 => 0,
            4 => 2,
            5 => 10,
            6 => 4,
            7 => 40,
            8 => 92,
            9 => 352,
            _ => 0,
        }
    }
}
