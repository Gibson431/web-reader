use super::RoyalRoadSource;
use crate::core::book::*;
use crate::core::chapter::*;
use crate::core::source::*;
use async_trait::async_trait;

const HOST: &str = "https://www.royalroad.com";

#[async_trait]
impl Source for RoyalRoadSource {
    fn as_str(&self) -> String {
        HOST.into()
    }

    async fn search(&self, term: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let search_ext = "/fictions/search?title=";
        let term = term.replace(" ", "+");
        let url = HOST.to_string() + search_ext + term.as_str();

        let document = RoyalRoadSource::get_document_from_url(url.clone()).await?;
        let binding = &scraper::Selector::parse(".fiction-list-item").unwrap();
        let search_entries = document.select(binding).into_iter();

        let mut results: Vec<String> = vec![];
        for e in search_entries {
            let url = e
                .select(&scraper::Selector::parse("h2").unwrap())
                .next()
                .unwrap()
                .select(&scraper::Selector::parse("a").unwrap())
                .next()
                .and_then(|a| a.value().attr("href"))
                .map(|s| HOST.to_owned() + s);

            let url = match url {
                Some(u) => u,
                None => continue,
            };

            results.push(url);
        }
        Ok(results)
    }

    async fn scrape_book(&self, url: String) -> Result<Book, Box<dyn std::error::Error>> {
        let document = RoyalRoadSource::get_document_from_url(url.clone()).await?;

        let name = document
            .select(&scraper::Selector::parse("h1.font-white").unwrap())
            .next()
            .map(|span| span.text().collect::<String>())
            .ok_or("Failed to retrieve name")?;

        let mut img = document
            .select(&scraper::Selector::parse(".thumbnail").unwrap())
            .next()
            .and_then(|img| img.value().attr("src"))
            .map(str::to_owned);

        if img == Some("/dist/img/nocover-new-min.png".to_owned()) {
            img = None;
        }

        let book = Book::new(self.as_str(), url, name, img);
        Ok(book)
    }

    async fn scrape_chapter(
        &self,
        url: String,
    ) -> Result<(Chapter, Option<String>), Box<dyn std::error::Error>> {
        let document = RoyalRoadSource::get_document_from_url(url.clone()).await?;
        let name = document
            .select(&scraper::Selector::parse(".break-word").unwrap())
            .next()
            .map(|h1| h1.text().collect::<String>());

        let next_chapter = document
            .select(&scraper::Selector::parse("i.far.fa-chevron-double-right.ml-3").unwrap())
            .next()
            .unwrap()
            .parent()
            .and_then(|a| scraper::ElementRef::wrap(a).unwrap().value().attr("href"))
            .map(str::to_string);

        Ok((Chapter::new(None, name, Some(url)), next_chapter))
    }

    async fn download_chapter(
        &self,
        chapter: &Chapter,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let Some(url) = &chapter.url else {
            return Err("No url available".into());
        };
        let document = RoyalRoadSource::get_document_from_url(url.clone()).await?;
        let content = document
            .select(&scraper::Selector::parse(".chapter-content")?)
            .next()
            .unwrap()
            .select(&scraper::Selector::parse("p")?)
            .map(|span| span.text().collect::<String>())
            .fold("".to_string(), |acc, c| acc + "\n" + &c);

        Ok(content)
    }
}

impl RoyalRoadSource {
    pub fn new() -> RoyalRoadSource {
        RoyalRoadSource
    }

    async fn get_document_from_url(
        url: String,
    ) -> Result<scraper::Html, Box<dyn std::error::Error>> {
        let response = reqwest::get(url).await;
        let html_content = response?.text().await?;
        Ok(scraper::Html::parse_document(&html_content))
    }

    // fn save_chapters_from_book(mut book: Book) -> Book {
    //     let mut chapters_to_scrape = Vec::<String>::new();
    //     let Some(url) = &book.url else {
    //         return book;
    //     };

    //     // get first chapter url
    //     let document = RoyalRoadSource::get_document_from_url(url.clone())?;
    //     let html_chapter_1 = document
    //         .select(&scraper::Selector::parse("tr.chapter-row").unwrap())
    //         .into_iter()
    //         .next()
    //         .unwrap();
    //     let chapter_1_url = html_chapter_1
    //         .select(&scraper::Selector::parse("td").unwrap())
    //         .next()
    //         .unwrap()
    //         .select(&scraper::Selector::parse("a").unwrap())
    //         .next()
    //         .and_then(|a| a.value().attr("href"))
    //         .unwrap();

    //     chapters_to_scrape.push(chapter_1_url.to_owned());
    //     while !chapters_to_scrape.is_empty() {
    //         // get the first element from the queue
    //         let chapter_to_scrape = chapters_to_scrape.remove(0);
    //         let document = RoyalRoadSource::get_document_from_url(chapter_to_scrape.clone());
    //         let content = document
    //             .select(&scraper::Selector::parse(".chapter-content").unwrap())
    //             .next()
    //             .unwrap()
    //             .select(&scraper::Selector::parse("p").unwrap())
    //             .map(|span| span.text().collect::<String>())
    //             .fold("".to_string(), |acc, c| acc + "\n" + &c);
    //         let name = document
    //             .select(&scraper::Selector::parse(".break-word").unwrap())
    //             .next()
    //             .map(|h1| h1.text().collect::<String>());

    //         let chapter = Chapter {
    //             name: name.clone(),
    //             url: Some(chapter_to_scrape),
    //             ..Default::default()
    //         };
    //         book.chapters.push(chapter.clone());

    //         fs::create_dir_all("books/".to_owned() + &book.name.clone()).unwrap();
    //         let mut f = File::create(
    //             "books/".to_owned() + &book.name.clone() + "/" + &name.unwrap().clone(),
    //         )
    //         .expect("Unable to create file");
    //         f.write_all(content.as_bytes())
    //             .expect("Unable to write data");

    //         println!("Saved chapter {:?}", &chapter.name);

    //         let next_chapter = document
    //             .select(&scraper::Selector::parse("i.far.fa-chevron-double-right.ml-3").unwrap())
    //             .next()
    //             .unwrap()
    //             .parent()
    //             .and_then(|a| scraper::ElementRef::wrap(a).unwrap().value().attr("href"));

    //         if let Some(c) = next_chapter {
    //             chapters_to_scrape.push(c.to_string());
    //         }
    //     }

    //     book
    // }
}
