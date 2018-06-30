extern crate termion;

use std::fmt;

pub struct Board {
    board: [[&'static str; 3]; 3]
}

impl Board {

    pub fn new() -> Board {
        Board {
            board: [[" ", " ", " "],
            [" ", " ", " "],
            [" ", " ", " "]]
        }
    }

    fn value(s: String) -> &'static str {
        if s == "X" {
            return "X";
        } else if s == "O" {
            return "O";
        } else {
            return " ";
        }
    }

    pub fn from_string(str_board: String) -> Board {
        let mut new_board = [[""; 3]; 3];
        let mut index = 0;
        for marker in str_board.chars() {
            new_board[index / 3][index % 3] = Board::value(marker.to_string());
            index += 1;
        }
        Board {
            board: new_board
        }
    }

    pub fn put(&mut self, row: usize, col: usize, token: &'static str) -> bool {
        if self.board[row][col] != " " {
            return false;
        }
        self.board[row][col] = token;
        true
    }

    pub fn remove(&mut self, row: usize, col: usize) -> bool {
        if self.board[row][col] == " " {
            return false;
        }
        self.board[row][col] = " ";
        true
    }

    pub fn available_moves(&self) -> Vec<Vec<usize>> {
        let mut available_moves = Vec::new();
        for row in 0..self.board.len() {
            for col in 0..self.board[0].len() {
                if self.board[row][col] == " " {
                    available_moves.push(vec!(row, col));
                }
            }
        }
        available_moves
    }

    pub fn is_over(&self) -> bool {
        self.is_filled() || self.someone_won()
    }

    pub fn is_filled(&self) -> bool {
        for row in 0..self.board.len() {
            for col in 0..self.board[0].len() {
                if self.board[row][col] == " " {
                    return false;
                }
            }
        }
        true
    }
    
    pub fn someone_won(&self) -> bool {
        self.board[0][0] != " " && self.board[0][0] == self.board[0][1] && self.board[0][0] == self.board[0][2] || 
            self.board[1][0] != " " && self.board[1][0] == self.board[1][1] && self.board[1][0] == self.board[1][2] ||
            self.board[2][0] != " " && self.board[2][0] == self.board[2][1] && self.board[2][0] == self.board[2][2] ||
            self.board[0][0] != " " && self.board[0][0] == self.board[1][0] && self.board[0][0] == self.board[2][0] ||
            self.board[0][1] != " " && self.board[0][1] == self.board[1][1] && self.board[0][1] == self.board[2][1] ||
            self.board[0][2] != " " && self.board[0][2] == self.board[1][2] && self.board[0][2] == self.board[2][2] ||
            self.board[0][0] != " " && self.board[0][0] == self.board[1][1] && self.board[0][0] == self.board[2][2] ||
            self.board[2][0] != " " && self.board[2][0] == self.board[1][1] && self.board[2][0] == self.board[0][2]
    }

    pub fn print(&self) {
        print!("{}", termion::clear::All);
        print!("{}", termion::cursor::Goto(1, 1));
        println!("{}", self);
    }

}

impl fmt::Display for Board {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str_board = String::from("");
        for row in 0..self.board.len() {
            for col in 0..self.board[0].len() {
                str_board.push_str(" ");
                if self.board[row][col] == "X" {
                    str_board.push_str(&format!("{}{}", termion::color::Fg(termion::color::Blue), self.board[row][col]));
                } else if self.board[row][col] == "O" {
                    str_board.push_str(&format!("{}{}", termion::color::Fg(termion::color::Red), self.board[row][col]));
                } else {
                    str_board.push_str(self.board[row][col]);
                }
                str_board.push_str(&format!("{}", termion::style::Reset));
                str_board.push_str(" ");
                if col < 2 {
                    str_board.push_str("|");
                }
            }
            str_board.push_str("\n");
            if row < 2 {
                str_board.push_str("-----------\n");
            }
        }

        writeln!(f, "{}", str_board)
    }    

}
