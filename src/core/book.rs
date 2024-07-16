use async_trait::async_trait;

use crate::core::chapter::Chapter;

#[derive(Debug, Clone, PartialEq)]
pub struct Book {
    pub source: String,
    pub url: String,
    pub image: Option<String>,
    pub name: String,
    pub chapters: Vec<Chapter>,
}

impl Book {
    pub fn new(source: String, url: String, name: String, image: Option<String>) -> Book {
        Book {
            source,
            url,
            image,
            name,
            chapters: vec![],
        }
    }

    pub async fn download_cover(&self) -> Result<bytes::Bytes, Box<dyn std::error::Error>> {
        let url = self.image.clone().ok_or("No image url")?;
        let response = reqwest::get(url).await?;
        let content = response.bytes().await?;
        Ok(content)
    }
}
