#[macro_use]
extern crate sciter;

mod ai;
mod board;
mod utils;

use ai::Ai;
use board::Board;

use sciter::Value;

use sciter::host::{Archive, LOAD_RESULT, SCN_LOAD_DATA};

struct EventHandler;
struct LoadHandler {
  archive: Archive,
}

impl LoadHandler {

    fn new(archive: &[u8]) -> Self {
        Self {
            archive: Archive::open(archive).expect("Unable to load archived resources"),
        }
    }

}

impl sciter::HostHandler for LoadHandler {

    fn on_data_load(&mut self, pnm: &mut SCN_LOAD_DATA) -> Option<LOAD_RESULT> {
        let uri = w2s!(pnm.uri);

        if uri.starts_with("this://app/") {
            if let Some(data) = self.archive.get(&uri) {
                self.data_ready(pnm.hwnd, &uri, data, None);
            } else {
                eprintln!("[handler] error: can't load {:?}", uri);
            }
        }
        return Some(LOAD_RESULT::LOAD_DEFAULT);
    }

}

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

    let resources = include_bytes!("resources.rc");
    let handler = LoadHandler::new(resources);

    let mut frame = sciter::Window::new();
    frame.event_handler(EventHandler);
    frame.sciter_handler(handler);
    frame.load_file("this://app/html/index.htm");
    frame.run_app();
}
