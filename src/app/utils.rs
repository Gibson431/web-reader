use std::collections::HashMap;
use std::path::Display;

use super::*;
use crate::core::book::Book;
use crate::fl;
use cosmic::app::{Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::window::Icon;
use cosmic::iced::{Alignment, Length, Padding, Size};
use cosmic::widget::*;
use cosmic::{cosmic_theme, theme, ApplicationExt, Apply, Element};

impl App {
    pub fn create_book_card(&self, book: &Book, size: Size) -> Element<Message> {
        let spacing = theme::active().cosmic().spacing;

        let mut card_content = cosmic::widget::column()
            .spacing(spacing.space_xxxs)
            .width(Length::Fill);
        // .height(Length::Shrink);

        let handle = self.get_image_handle(book);

        card_content = card_content.push(
            cosmic::iced::widget::image(handle.clone())
                .content_fit(cosmic::iced::ContentFit::Contain)
                .width(Length::Fill)
                .border_radius([spacing.space_xxs as f32; 4])
                .apply(container)
                .style(cosmic::theme::Container::Secondary),
        );

        card_content = card_content.push(
            cosmic::widget::text(book.name.clone()).height(Length::Fixed(spacing.space_xl as f32)),
        );

        let card = container(card_content)
            .padding(spacing.space_xxs)
            .style(cosmic::theme::Container::Secondary);

        let button = widget::button::custom_image_button(card, None)
            .on_press(Message::ToggleContextPage(ContextPage::BookContext(
                book.clone(),
            )))
            .style(cosmic::theme::Button::Image)
            .width(Length::Fixed(size.width))
            .height(Length::Shrink);

        button.into()
    }

    pub async fn download_book_cover(
        image_url: String,
    ) -> Result<bytes::Bytes, Box<dyn std::error::Error>> {
        // let url = book.image.clone().ok_or("No image url")?;
        let response = reqwest::blocking::get(image_url)?;
        let content = response.bytes()?;
        _ = cosmic::widget::image::Handle::from_memory(content.clone());
        Ok(content)
    }

    pub fn get_image_handle(&self, book: &Book) -> cosmic::widget::image::Handle {
        if let Some(h) = self.book_covers.get(&book.url) {
            cosmic::widget::image::Handle::from_memory(h.clone())
        } else {
            if let Ok(opt) = self.get_thumbnail_from_storage(book.clone()) {
                if let Some(bytes) = opt {
                    return cosmic::widget::image::Handle::from_memory(bytes);
                };
            }
            cosmic::widget::image::Handle::from_path("res/covers/rr-image.png")
        }
    }

    pub fn log_error(&self, err: String) -> Command<Message> {
        Command::perform(
            async move { message::app(Message::Log(LogMessage::Error(err))) },
            |x| x,
        )
    }

    pub fn get_book_from_storage(
        &self,
        url: String,
    ) -> Result<Option<Book>, Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open(self.storage_path.join(STORAGE_FILE))?;
        let mut stmt = conn
            .prepare("SELECT source, url, name, image, in_library FROM books WHERE url = :url;")?;

        let book_iter = stmt.query_map(&[(":url", &url)], |row| {
            Ok(Book::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        for book in book_iter {
            let mut book = book.unwrap();
            if book.image == Some("".into()) {
                book.image = None;
            }
            return Ok(Some(book));
        }

        Ok(None)
    }

    pub fn get_thumbnail_from_storage(
        &self,
        book: Book,
    ) -> Result<Option<bytes::Bytes>, Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open(self.storage_path.join(STORAGE_FILE))?;
        let mut stmt = conn.prepare("SELECT image FROM thumbnails WHERE url = :url;")?;

        let image_iter = stmt.query_map(&[(":url", &book.url)], |row| {
            Ok(row.get::<usize, Vec<u8>>(0)?)
        })?;

        for img in image_iter {
            let bytes = bytes::Bytes::from(img?);
            return Ok(Some(bytes));
        }

        Ok(None)
    }

    pub fn send_thumbnail_to_storage(&self, book: Book) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = match self.book_covers.get(&book.url) {
            Some(b) => b,
            None => {
                dbg!(&book);
                return Err("No book cover".into()).into();
            }
        };

        let conn = rusqlite::Connection::open(self.storage_path.join(STORAGE_FILE))?;
        let _ = conn.execute(
            "INSERT INTO thumbnails (url, image) values (?1, ?2)",
            (&book.url.clone(), &bytes.to_vec()),
        )?;

        Ok(())
    }
}
