impl Solution {
    pub fn convert(s: String, num_rows_i: i32) -> String {
        let num_rows: usize = num_rows_i as usize;
        
        if num_rows == 1 {
            return s;
        }
        
        let mut rows: Vec<Vec<char>> = Vec::with_capacity(num_rows);
        for i in 0..num_rows {
            rows.push(Vec::new());
        }
        
        for i in 0..s.len() {
            let r = i / (num_rows - 1);
            let c = i % (num_rows - 1);
            
            let ch = s.chars().nth(i).unwrap();
            
            if r % 2 == 0 {
                rows[c].push(ch);
            } else {
                rows[num_rows - 1 - c].push(ch);
            }
        }
        
        let mut ans: String = String::new();
        for i in 0..num_rows {
            for j in 0..rows[i].len() {
                ans.push(rows[i][j]);
            }
        }
        
        ans
    }
}
