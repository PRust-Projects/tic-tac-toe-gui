use board;
use utils;

pub const TYPE: &str = "Ai";

pub struct Ai {
    token: &'static str,
    player_type: &'static str
}

impl Ai {

    pub fn new(token: &'static str) -> Ai {
        Ai {
            token: token,
            player_type: TYPE
        }
    }

    pub fn get_token(&self) -> &'static str {
        self.token
    }

    pub fn get_type(&self) -> &'static str {
        self.player_type
    }

    pub fn get_move(&self, board: &mut board::Board) -> [usize; 2] {
        let (best_move, _score) = self.minimax(board, 0);
        [best_move[0], best_move[1]]
    }
    
    fn minimax(&self, board: &mut board::Board, turn: i32) -> (Vec<usize>, i32) {
        if board.someone_won() {
            if turn == 0 {
                return (Vec::new(), -10); 
            } else {
                return (Vec::new(), 10);
            }
        }
        if board.is_filled() {
            return (Vec::new(), 0);
        }

        let available_moves = board.available_moves();
        let mut moves = Vec::new();
        let mut scores = Vec::new();
        for available_move in &available_moves {
            let row = available_move[0];
            let col = available_move[1];
            if turn == 0 {
                board.put(row, col, self.token);
            } else {
                board.put(row, col, utils::get_other_token(self.token));
            }
            moves.push(available_move);
            let (_best_move, score) = self.minimax(board, 1-turn);
            scores.push(score);
            board.remove(row, col);
        }
        let (best_score, index) = self.get_best_move(scores, turn);
        return (moves[index].to_vec(), best_score);
    }

    fn get_best_move(&self, scores: Vec<i32>, turn: i32) -> (i32, usize) {
        if turn == 0 {
            let max_index = self.get_max_index(&scores);
            return (scores[max_index], max_index);
        } else {
            let min_index = self.get_min_index(&scores);
            return (scores[min_index], min_index);
        }
    }

    fn get_max_index(&self, vec: &Vec<i32>) -> usize {
        let mut max_index = 0;
        for index in 1..vec.len() {
            if vec[index] > vec[max_index] {
                max_index = index;
            }
        }
        max_index
    }

    fn get_min_index(&self, vec: &Vec<i32>) -> usize {
        let mut min_index = 0;
        for index in 1..vec.len() {
            if vec[index] < vec[min_index] {
                min_index = index;
            }
        }
        min_index
    }

}
