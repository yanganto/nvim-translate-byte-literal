extern crate neovim_lib;

#[cfg(test)]
mod tests;

use std::str;

use neovim_lib::{Neovim, NeovimApi, Session};

struct Engine;

impl Engine {
    fn new() -> Engine {
        Engine {}
    }

    fn translate_number_array(&self, nums: Vec<u8>) -> String {
        str::from_utf8(&nums)
            .unwrap_or("Translate fail".into())
            .into()
    }

    fn str_preprocessor(&self, raw_str: &str) -> Vec<u8> {
        let mut output = Vec::new();
        for s in raw_str
            .split_at(raw_str.find('[').unwrap_or_default())
            .1
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(",")
        {
            if let Ok(n) = s.trim().parse::<u8>() {
                output.push(n);
            }
        }
        output
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
                            "echo {:?}",
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
