pub fn run() -> bool {
    let board = vec![
    vec!['A', 'B', 'C', 'E'],
    vec!['S','F','C','S'],
    vec!['A','D','E','E']
    ];

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        use std::collections::HashSet;

        type Coord = (usize, usize);

        if word.is_empty() { return false; }
        if board.is_empty() { return false; }
        if board[0].is_empty() { return false; }

        let word = word.chars().collect::<Vec<char>>();
        let mut visited: HashSet<Coord> = HashSet::new();
        fn recursive_search(
            board: &Vec<Vec<char>>, 
            word: &Vec<char>, 
            visited: &mut HashSet<Coord>,
            len: usize,
            coord: Coord
        ) -> bool {
            if board[coord.0][coord.1] != word[len] { return false; }

            visited.insert(coord.clone());
            let len = len + 1;
            if len == word.len() { return true; }

            let max = (board.len() - 1, board[0].len() - 1);

            if coord.0 > 0 && !visited.contains(&(coord.0 - 1, coord.1)) {
                if recursive_search(board, word, visited, len, 
                    (coord.0 - 1, coord.1)) { return true; }
            }
            if coord.0 < max.0 && !visited.contains(&(coord.0 + 1, coord.1)) {
                if recursive_search(board, word, visited, len, 
                    (coord.0 + 1, coord.1)) { return true; }
            }
            if coord.1 > 0 && !visited.contains(&(coord.0, coord.1 - 1)) {
                if recursive_search(board, word, visited, len, 
                    (coord.0, coord.1 - 1)) { return true; }
            }
            if coord.1 < max.1 && !visited.contains(&(coord.0, coord.1 + 1)) {
                if recursive_search(board, word, visited, len, 
                    (coord.0, coord.1 + 1)) { return true; }
            }
            visited.remove(&coord);
            false
        }


        let max = (board.len() - 1, board[0].len() - 1);

        for i in 0..=max.0 {
            for j in 0..=max.1 {
                if recursive_search(&board, &word, &mut visited, 0, (i,j)) {
                    return true;
                }
            }
        }

        false
    }

    exist(board, String::from("ASADFBCCEESE"))
}