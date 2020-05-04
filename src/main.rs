extern crate neovim_lib;

#[cfg(test)]
mod tests;

use neovim_lib::{Neovim, NeovimApi, Session};

struct Engine;

impl Engine {
    fn new() -> Engine {
        Engine {}
    }

    fn translate_number_array(&self, nums: Vec<u64>) -> String {
        "Not Implement".to_string()
    }

    fn str_preprocessor(&self, raw_str: &str) -> Vec<u64> {
        vec![]
    }
}

enum Messages {
    TranslateByteArray,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "TranslateByteArray" => Messages::TranslateByteArray,
            _ => Messages::Unknown(event),
        }
    }
}

struct EventHandler {
    nvim: Neovim,
    engine: Engine,
}

impl EventHandler {
    fn new() -> EventHandler {
        let session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);
        let engine = Engine::new();

        EventHandler { nvim, engine }
    }

    fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, values) in receiver {
            match Messages::from(event) {
                Messages::TranslateByteArray => {
                    let nums = self
                        .engine
                        .str_preprocessor(values[0].as_str().unwrap_or_default());
                    self.nvim
                        .command(&format!(
                            "echo \"{}\"",
                            self.engine.translate_number_array(nums)
                        ))
                        .unwrap();
                }

                // Handle anything else
                Messages::Unknown(event) => {
                    self.nvim
                        .command(&format!("echo \"Unknown command: {}\"", event))
                        .unwrap();
                }
            }
        }
    }
}

fn main() {
    let mut event_handler = EventHandler::new();
    event_handler.recv();
}
