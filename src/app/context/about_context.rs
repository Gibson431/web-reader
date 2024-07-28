use std::collections::HashMap;

use crate::app::{App, Message, REPOSITORY};
use crate::core::source::{self, *};
use crate::core::{self, Book, Chapter};
use crate::fl;
use cosmic::app::{message, Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, *};
use cosmic::{cosmic_theme, theme, ApplicationExt, Apply, Element};

impl App {
    /// The about page for this app.
    pub fn about_context(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon_style = cosmic::iced::widget::svg::Appearance {
            color: Some(cosmic::iced::Color::WHITE),
        };
        // let icon_style = cosmic::iced_widget::Theme::Light;
        let icon = widget::svg(widget::svg::Handle::from_memory(
            &include_bytes!("../../../res/icons/hicolor/48x48/apps/settings-svgrepo-com.svg")[..],
        ))
        .height(48)
        .width(48);

        let title = widget::text::title3(fl!("app-title"));

        let title_row = widget::row()
            .push(icon)
            .push(title)
            .spacing(space_xxs)
            .width(Length::Shrink)
            .align_items(Alignment::Center);

        let link = widget::button::link(REPOSITORY)
            .on_press(Message::LaunchUrl(REPOSITORY.to_string()))
            .padding(0);

        widget::column()
            .push(title_row)
            .push(link)
            .align_items(Alignment::Center)
            .spacing(space_xxs)
            .width(Length::Fill)
            .into()
    }
}
