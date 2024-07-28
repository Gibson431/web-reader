use crate::app::{App, Message};
use cosmic::iced::{Alignment, Length, Padding, Size};
use cosmic::{theme, Apply, Element};

pub trait Page: Send + Sync {
    fn view(&self, app: &App, size: Size) -> Element<Message> {
        todo!();
    }
}
