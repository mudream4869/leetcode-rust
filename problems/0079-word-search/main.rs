impl Solution {
    pub fn _exist(board: &Vec<Vec<char>>, word: &Vec<char>, word_ptr: usize, status: &mut Vec<Vec<bool>>, x: i32, y: i32) -> bool {
        if word_ptr == word.len() {
            return true;
        }
        
        if x < 0 || y < 0 || x >= board.len() as i32 || y >= board[0].len() as i32 {
            return false;
        }
        
        let ux = x as usize;
        let uy = y as usize;

        if word[word_ptr] != board[ux][uy] || status[ux][uy] {
            return false;
        }
        
        status[ux][uy] = true;
        let dirs: Vec<(i32, i32)> = vec![
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0)
        ];
        for (dx, dy) in dirs {
            if Self::_exist(board, word, word_ptr + 1, status, x + dx, y + dy) {
                status[ux][uy] = false;
                return true;
            }
        }
        status[ux][uy] = false;
        return false;
    }
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word_chs = word.chars().collect::<Vec<_>>();
        let mut status = vec![vec![false; board[0].len()]; board.len()];
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Self::_exist(&board, &word_chs, 0, &mut status, i as i32, j as i32) {
                    return true;
                }
            }
        }
        false
    }
}
