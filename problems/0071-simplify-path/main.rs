impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut ans_vec = Vec::new();
        for dir in path.split("/") {
            if dir == ".." {
                ans_vec.pop();
                continue;
            } else if dir == "" || dir == "." {
                continue;
            }
            
            ans_vec.push(dir);
        }
        
        "/".to_string() + &ans_vec.join("/")
    }
}
