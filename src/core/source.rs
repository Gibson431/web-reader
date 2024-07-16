pub mod royalroad;
use super::book::Book;
use super::chapter::Chapter;
use async_trait::async_trait;

#[async_trait]
pub trait Source: Send + Sync {
    fn as_str(&self) -> String;
    async fn search(&self, term: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;

    async fn scrape_book(&self, url: String) -> Result<Book, Box<dyn std::error::Error>>;
    async fn scrape_chapter(
        &self,
        url: String,
    ) -> Result<(Chapter, Option<String>), Box<dyn std::error::Error>>;
    async fn download_chapter(
        &self,
        chapter: &Chapter,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

use core::fmt::Debug;
impl Debug for dyn Source {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Source{{{}}}", self.as_str())
    }
}

#[derive(Default)]
pub struct RoyalRoadSource;
