// SPDX-License-Identifier: GPL-3.0-only

pub mod book;
pub mod chapter;
pub mod data;
pub mod localization;
pub mod source;
pub mod widget;

#[derive(Debug, Clone, PartialEq)]
pub struct Book {
    pub source: String,
    pub url: String,
    pub image: Option<String>,
    pub name: String,
    pub in_library: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Chapter {
    pub number: Option<u32>,
    pub name: Option<String>,
    pub url: Option<String>,
}
