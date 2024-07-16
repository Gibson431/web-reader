use std::collections::HashMap;
use std::path::Display;

use crate::app::*;
use crate::core::book::Book;
use crate::fl;
use cosmic::app::{Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::window::Icon;
use cosmic::iced::{Alignment, Length, Padding, Size};
use cosmic::widget::*;
use cosmic::{cosmic_theme, theme, ApplicationExt, Apply, Element};

impl App {
    pub fn view_library(&self, _size: Size) -> Element<Message> {
        let spacing = theme::active().cosmic().spacing;

        let search_bar = cosmic::widget::row()
            .align_items(Alignment::Center)
            .spacing(spacing.space_xs)
            .push(
                cosmic::widget::search_input("Search for books...", &self.library_input)
                    .width(Length::Fill)
                    .on_input(Message::LibraryInputChanged)
                    .on_submit_maybe(Some(Message::LibrarySearch(self.library_input.clone()))),
            )
            .apply(container);

        let content = cosmic::widget::container("No results")
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(spacing.space_xxs)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .style(cosmic::theme::Container::default());

        column()
            .push(search_bar)
            .push(content)
            .spacing(spacing.space_xxs)
            .apply(container)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
