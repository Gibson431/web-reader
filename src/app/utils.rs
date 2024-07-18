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

        let handle = self.data_manager.get_image_handle(book);

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
        let response = reqwest::blocking::get(image_url)?;
        let content = response.bytes()?;
        _ = cosmic::widget::image::Handle::from_memory(content.clone());
        Ok(content)
    }

    pub fn log_error(&self, err: String) -> Command<Message> {
        Command::perform(
            async move { message::app(Message::Log(LogMessage::Error(err))) },
            |x| x,
        )
    }
}
