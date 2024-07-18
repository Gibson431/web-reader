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
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

impl App {
    pub fn view_library(&self, size: Size) -> Element<Message> {
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
                cosmic::widget::search_input("Search for books...", &self.library_input)
                    .width(Length::Fill)
                    .on_input(Message::LibraryInputChanged)
                    .on_submit_maybe(Some(Message::Ignore)),
                // .on_submit_maybe(Some(Message::LibrarySearch(self.library_input.clone()))),
            )
            .apply(container);

        let books = if self.library_input.is_empty() {
            match self.get_library_books() {
                Ok(b) => b,
                Err(e) => {
                    dbg!(e);
                    vec![]
                }
            }
        } else {
            match self.get_library_books_like(self.library_input.clone()) {
                Ok(b) => b,
                Err(e) => {
                    dbg!(e);
                    vec![]
                }
            }
        };

        let content;
        if books.is_empty() {
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
            for (_i, book) in books.clone().into_iter().enumerate() {
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
            .apply(container)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }

    fn get_library_books(&self) -> Result<Vec<Book>, Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open(self.storage_path.join(STORAGE_FILE))?;
        let mut stmt = conn
            .prepare("SELECT source, url, name, image, in_library FROM books WHERE in_library = 1")
            .unwrap();

        let book_iter = stmt
            .query_map([], |row| {
                Ok(Book::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            })
            .unwrap();

        let mut books = vec![];
        for book in book_iter {
            match book {
                Ok(mut book) => {
                    if book.image == Some("".into()) {
                        book.image = None;
                    }
                    books.push(book);
                }
                Err(e) => {
                    dbg!(e);
                }
            }
        }

        Ok(books)
    }

    fn get_library_books_like(
        &self,
        search_term: String,
    ) -> Result<Vec<Book>, Box<dyn std::error::Error>> {
        let books = self.get_library_books()?;
        let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();
        let mut books = books
            .iter()
            .map(|b| (matcher.fuzzy_match(&b.name, &search_term), b))
            .filter_map(|(score_opt, b)| match score_opt {
                Some(s) => Some((s, b.clone())),
                None => None,
            })
            .collect::<Vec<(i64, Book)>>();
        books.sort_by(|a, b| b.0.cmp(&a.0));
        let books = books.iter().map(|(_, b)| b.clone()).collect::<Vec<Book>>();
        Ok(books)
    }
}
