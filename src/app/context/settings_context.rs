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
    /// The settings page for this app.
    pub fn settings_context(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon = widget::svg(widget::svg::Handle::from_memory(
            &include_bytes!("../../../res/icons/hicolor/48x48/apps/settings-svgrepo-com.svg")[..],
        ))
        .height(64);

        let display_options = widget::column()
            .push(widget::row().push(widget::text("Display")))
            .align_items(Alignment::Center);
        let contact_info = widget::column()
            .push(
                widget::button::link(REPOSITORY)
                    .on_press(Message::LaunchUrl(REPOSITORY.to_string()))
                    .padding(0),
            )
            .align_items(Alignment::Center);

        let clear_storage_btn = widget::button("Clear Storage").on_press(Message::ClearStorage);

        widget::column()
            .push(icon)
            .push(widget::divider::horizontal::default())
            .push(display_options)
            .push(widget::divider::horizontal::default())
            .push(contact_info)
            .push(widget::divider::horizontal::default())
            .push(clear_storage_btn)
            .align_items(Alignment::Center)
            .spacing(space_xxs)
            .into()
    }
}
