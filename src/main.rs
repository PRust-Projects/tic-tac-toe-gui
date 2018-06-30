#[macro_use]
extern crate sciter;

mod ai;
mod board;
mod utils;

use ai::Ai;
use board::Board;

use sciter::Value;

struct EventHandler;

impl EventHandler {
    
    fn get_best_move(&self, board: String, token: String) -> Value {
        let token = if token == "X" {
            "X"
        } else {
            "O"
        };
        let mut board = Board::from_string(board);
        let ai = Ai::new(token);
        let best_move = ai.get_move(&mut board);
        varray![best_move[0] as i32, best_move[1] as i32]
    }

}

impl sciter::EventHandler for EventHandler {

    dispatch_script_call! {
        fn get_best_move(String, String);
    }

}

fn main() {
    sciter::set_options(sciter::RuntimeOptions::DebugMode(true));
    let mut frame = sciter::Window::new();
    frame.event_handler(EventHandler);
    frame.load_file("resources/html/minimal.htm");
    frame.run_app();
}
