use crate::command::Command;

mod key_handlers;
pub use key_handlers::{KeyHandler, KeyHandlers};

pub enum KeyEventResult {
    Consumed(Option<Command>),
    Ignored,
}