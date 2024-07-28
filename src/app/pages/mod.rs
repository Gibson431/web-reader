// use crate::core::page::Entity;

pub mod explore;
pub mod history;
pub mod library;
pub mod page;
pub mod reading;

// #[derive(Clone, Debug)]
// pub enum Message {
//     Explore(explore::Message),
//     External { id: String, message: Vec<u8> },
//     Page(Entity),
// }

// impl From<Message> for crate::app::Message {
//     fn from(message: Message) -> Self {
//         crate::app::Message::PageMessage(message)
//     }
// }
