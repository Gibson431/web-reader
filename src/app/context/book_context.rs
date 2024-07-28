use std::collections::HashMap;

use crate::app::{App, Message};
use crate::core::source::{self, *};
use crate::core::{self, Book, Chapter};
use crate::fl;
use cosmic::app::{message, Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, *};
use cosmic::{cosmic_theme, theme, ApplicationExt, Apply, Element};

impl App {
    // The book context page
    pub fn book_context(&self, mut book: Book) -> Element<Message> {
        let spacing = theme::active().cosmic().spacing;

        match self.data_manager.get_book_from_storage(book.url.clone()) {
            Ok(b) => {
                if let Some(b) = b {
                    book = b;
                }
            }
            Err(e) => {
                dbg!(e);
            }
        }

        let image = widget::image(self.data_manager.get_image_handle(&book))
            .content_fit(cosmic::iced::ContentFit::Contain)
            .border_radius([spacing.space_xxs as f32; 4])
            .apply(container)
            .max_height(200)
            .max_width(200);

        let title_row = widget::row()
            .push(image)
            .push(
                widget::column()
                    .push(widget::text(book.name.clone()))
                    .push(widget::text(book.source.clone()).style(cosmic::theme::Text::Default))
                    .spacing(spacing.space_xxs),
            )
            .spacing(spacing.space_xxs)
            .align_items(Alignment::Center)
            .width(Length::Shrink)
            .apply(container)
            .align_x(Horizontal::Left)
            .width(Length::Fill);

        let interaction_row = widget::row()
            .push(
                widget::button::button(if book.in_library {
                    "In Library"
                } else {
                    "Not Library"
                })
                .on_press(Message::LibraryToggle(book.clone()))
                .padding(spacing.space_xxs),
            )
            .push(
                widget::button::button(widget::text(fl!("site")))
                    .on_press(Message::LaunchUrl(book.url.clone()))
                    .padding(spacing.space_xxs),
            )
            .push(
                widget::button("Refresh")
                    .on_press(Message::RefreshBook(book.url.clone()))
                    .padding(spacing.space_xxs),
            )
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .apply(container)
            .width(Length::Fill)
            .align_x(Horizontal::Center);

        let link = widget::button::link(book.url.clone())
            .on_press(Message::LaunchUrl(book.url.clone()))
            .padding(0);

        let mut chapters: Vec<Element<Message>> = vec![];
        for i in 0..15 {
            chapters.push(
                widget::button::button(widget::text(format!("Chapter {}", i + 1)))
                    .on_press(Message::ReadChapter(Chapter::new(None, None, None)))
                    .into(),
            );
        }

        let chapter_view = widget::column()
            .push(widget::text("Chapters"))
            .push(widget::divider::horizontal::default())
            .push(
                widget::container(
                    widget::column::with_children(chapters)
                        .width(Length::Fill)
                        .apply(scrollable),
                )
                .height(Length::Fixed(200 as f32))
                .width(Length::Fill),
            )
            .align_items(Alignment::Start)
            .spacing(spacing.space_xxs)
            .apply(container)
            .style(cosmic::theme::Container::Card)
            .padding(spacing.space_xs);

        widget::column()
            // .push(icon)
            // .push(title)
            .push(title_row)
            .push(widget::divider::horizontal::default())
            .push(interaction_row)
            .push(widget::divider::horizontal::default())
            .push(chapter_view)
            .push(link)
            .align_items(Alignment::Center)
            .spacing(spacing.space_xs)
            .into()
    }
}
