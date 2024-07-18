use crate::app::*;

use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::{Alignment, Length, Padding, Size};
use cosmic::widget::*;
use cosmic::{theme, Apply, Element};

impl App {
    pub fn view_explore(&self, size: Size) -> Element<Message> {
        let spacing = theme::active().cosmic().spacing;
        let item_width = 180;
        let item_height = 400;
        let (width, _height) = (
            (size.width.floor() as usize)
                .checked_sub(spacing.space_s as usize)
                .unwrap_or(0)
                .max(item_width),
            (size.height.floor() as usize).max(item_height),
        );

        let (cols, column_spacing) = {
            let width_m1 = width.checked_sub(item_width).unwrap_or(0);
            let cols_m1 = width_m1 / (item_width + spacing.space_xxs as usize);
            let cols = cols_m1 + 1;
            let spacing = width_m1
                .checked_div(cols_m1)
                .unwrap_or(0)
                .checked_sub(item_width)
                .unwrap_or(0);
            (cols, spacing as u16)
        };

        let search_bar = cosmic::widget::row()
            .align_items(Alignment::Center)
            .spacing(spacing.space_xs)
            .push(
                cosmic::widget::search_input("Search for books...", &self.explore_input)
                    .width(Length::Fill)
                    .on_input(Message::ExploreInputChanged)
                    .on_submit_maybe(Some(Message::ExploreSearch(self.explore_input.clone()))),
            )
            .apply(container);

        let content;
        if self.explore_results.is_empty() {
            content = cosmic::widget::container("No results")
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(spacing.space_xxs)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .style(cosmic::theme::Container::default());
        } else {
            let mut grid = cosmic::widget::grid()
                .width(Length::Fill)
                .column_spacing(column_spacing)
                .row_spacing(spacing.space_xs)
                .insert_row();

            let card_size = Size::new(item_width as f32, item_height as f32);
            let mut col = 0;
            for (_i, url) in self.explore_results.clone().into_iter().enumerate() {
                let book = match self.data_manager.get_book(&url) {
                    Ok(Some(b)) => b.clone(),
                    _ => continue,
                };
                grid = grid.push(self.create_book_card(&book, card_size));
                col += 1;
                if col >= cols {
                    col = 0;
                    grid = grid.insert_row();
                }
            }

            let grid = grid
                .apply(container)
                .center_x()
                .height(Length::Fill)
                .width(Length::Fill)
                .padding(Padding {
                    top: 0.0,
                    bottom: 0.0,
                    left: spacing.space_xs as f32,
                    right: spacing.space_m as f32,
                })
                .apply(scrollable)
                .height(Length::Fill)
                .width(Length::Fill);

            content = container::Container::new(grid);
        }

        column()
            .push(search_bar)
            .push(content)
            .spacing(spacing.space_xxs)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
