use async_trait::async_trait;

use crate::core::Chapter;

use super::Book;

impl Book {
    pub fn new(
        source: String,
        url: String,
        name: String,
        image: Option<String>,
        in_library: bool,
    ) -> Book {
        Book {
            source,
            url,
            image,
            name,
            in_library,
        }
    }

    pub async fn download_cover(&self) -> Result<bytes::Bytes, Box<dyn std::error::Error>> {
        let url = self.image.clone().ok_or("No image url")?;
        let response = reqwest::get(url).await?;
        let content = response.bytes().await?;
        Ok(content)
    }
}
