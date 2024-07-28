use crate::core::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct DataManager {
    storage_path: std::path::PathBuf,
    books: HashMap<String, Book>,                   // book url, book
    pub book_covers: HashMap<String, bytes::Bytes>, // book url, bytes
}

impl DataManager {
    const STORAGE_FILE: &'static str = "data.db";

    pub fn new() -> DataManager {
        DataManager::default()
    }

    pub fn clear_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.storage_path.join(Self::STORAGE_FILE).exists() {
            if let Err(e) = std::fs::remove_file(self.storage_path.join(Self::STORAGE_FILE)) {
                return Err(e.into());
            }
        }
        let res = self.init(self.storage_path.clone());
        if !res.is_empty() {
            return Err("Failed to reinit storage".into());
        }
        Ok(())
    }

    pub fn init(&mut self, dir: std::path::PathBuf) -> Vec<Box<dyn std::error::Error>> {
        self.storage_path = dir;

        if !self.storage_path.exists() {
            if let Err(e) = std::fs::create_dir(&self.storage_path) {
                return vec![e.into()];
            };
        }

        let mut errors = vec![];
        match rusqlite::Connection::open(self.storage_path.join(Self::STORAGE_FILE)) {
            Ok(conn) => {
                let mut errors = vec![];

                // Create books table
                if let Err(e) = conn.execute(
                    "CREATE TABLE if not exists books (
                    id INTEGER PRIMARY KEY,
                    source TEXT,
                    name TEXT, 
                    book_url TEXT, 
                    image_url TEXT, 
                    in_library BIT);",
                    (),
                ) {
                    errors.push(e);
                };

                // Create thumbnails table
                if let Err(e) = conn.execute(
                    "CREATE TABLE if not exists thumbnails (
                    id INTEGER PRIMARY KEY,
                    book_url TEXT, 
                    image_data BLOB);",
                    (),
                ) {
                    errors.push(e);
                };

                // Create chapters table
                if let Err(e) = conn.execute(
                    "CREATE TABLE if not exists chapters (
                    id INTEGER PRIMARY KEY,
                    book_url TEXT,
                    name TEXT, 
                    chapter_url TEXT, 
                    release_date TEXT);",
                    (),
                ) {
                    errors.push(e);
                };
            }
            Err(e) => errors.push(e.into()),
        };
        errors
    }

    pub fn get_book_from_storage(
        &self,
        url: String,
    ) -> Result<Option<Book>, Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open(self.storage_path.join(Self::STORAGE_FILE))?;
        let mut stmt = conn.prepare(
            "SELECT source, book_url, name, image_url, in_library FROM books WHERE book_url = :url;",
        )?;

        let book_iter = stmt.query_map(&[(":url", &url)], |row| {
            Ok(Book::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        for book in book_iter {
            let mut book = book.unwrap();
            if book.image == Some("".into()) {
                book.image = None;
            }
            return Ok(Some(book));
        }

        Ok(None)
    }

    pub fn get_book(&self, url: &String) -> Result<Option<Book>, Box<dyn std::error::Error>> {
        if let Some(book) = self.books.get(url) {
            return Ok(Some(book.clone()));
        }

        let conn = rusqlite::Connection::open(self.storage_path.join(Self::STORAGE_FILE))?;
        let mut stmt = conn.prepare(
            "SELECT source, book_url, name, image_url, in_library FROM books WHERE book_url = :url;",
        )?;

        let mut book_iter = stmt.query_map(&[(":url", &url)], |row| {
            Ok(Book::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        match book_iter.next() {
            Some(Ok(mut book)) => {
                if book.image == Some("".into()) {
                    book.image = None;
                }
                return Ok(Some(book));
            }
            Some(Err(e)) => return Err(e.into()),
            None => return Ok(None),
        }
    }

    pub fn set_book(&mut self, book: &Book) -> Result<(), Box<dyn std::error::Error>> {
        match self.get_book(&book.url) {
            Ok(Some(_)) => self.update_book(book.clone()),
            _ => self.add_book(book.clone()),
        }?;

        Ok(())
    }

    fn add_book(&mut self, book: Book) -> Result<(), Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open(self.storage_path.join(Self::STORAGE_FILE))?;
        conn.execute(
            "INSERT INTO books (source, name, book_url, image_url, in_library) values (?1, ?2, ?3 ,?4, ?5)",
            [
                book.source.clone(),
                book.name.clone(),
                book.url.clone(),
                book.image.clone().unwrap_or("".into()),
                if book.in_library {
                    "1".into()
                } else {
                    "0".into()
                },
            ],
        )?;

        _ = self.books.insert(book.url.clone(), book.clone());
        Ok(())
    }
    fn update_book(&mut self, book: Book) -> Result<(), Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open(self.storage_path.join(Self::STORAGE_FILE))?;
        _ = conn.execute(
            "UPDATE books SET source = ?1, name = ?2, book_url = ?3, image_url = ?4, in_library = ?5 WHERE book_url = ?3;",
            [
                book.source.clone(),
                book.name.clone(),
                book.url.clone(),
                book.image.clone().unwrap_or("".into()),
                if book.in_library {
                    "1".into()
                } else {
                    "0".into()
                },
            ],
        )?;

        _ = self.books.insert(book.url.clone(), book.clone());
        Ok(())
    }

    pub fn get_library_books(&self) -> Result<Vec<Book>, Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open(self.storage_path.join(Self::STORAGE_FILE))?;
        let mut stmt = conn
            .prepare("SELECT source, book_url, name, image_url, in_library FROM books WHERE in_library = 1")
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

    pub fn get_image_handle(&self, book: &Book) -> cosmic::widget::image::Handle {
        if let Some(h) = self.book_covers.get(&book.url) {
            cosmic::widget::image::Handle::from_memory(h.clone())
        } else {
            if let Ok(Some(bytes)) = self.get_image_as_bytes(book) {
                return cosmic::widget::image::Handle::from_memory(bytes);
            }
            cosmic::widget::image::Handle::from_path("res/covers/rr-image.png")
        }
    }

    pub fn get_image_as_bytes(
        &self,
        book: &Book,
    ) -> Result<Option<bytes::Bytes>, Box<dyn std::error::Error>> {
        if let Some(h) = self.book_covers.get(&book.url) {
            return Ok(Some(h.clone()));
        };

        let conn = rusqlite::Connection::open(self.storage_path.join(Self::STORAGE_FILE))?;
        let mut stmt = conn.prepare("SELECT image_data FROM thumbnails WHERE book_url = :url;")?;

        let mut image_iter = stmt.query_map(&[(":url", &book.url)], |row| {
            Ok(row.get::<usize, Vec<u8>>(0)?)
        })?;

        return match image_iter.next() {
            Some(img) => {
                let bytes = bytes::Bytes::from(img?);
                Ok(Some(bytes))
            }
            None => Ok(None),
        };
    }

    fn get_image_from_storage(
        &self,
        book: &Book,
    ) -> Result<Option<bytes::Bytes>, Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open(self.storage_path.join(Self::STORAGE_FILE))?;
        let mut stmt = conn.prepare("SELECT image_data FROM thumbnails WHERE book_url = :url;")?;

        let mut image_iter = stmt.query_map(&[(":url", &book.url)], |row| {
            Ok(row.get::<usize, Vec<u8>>(0)?)
        })?;

        return match image_iter.next() {
            Some(img) => {
                let bytes = bytes::Bytes::from(img?);
                Ok(Some(bytes))
            }
            None => Ok(None),
        };
    }

    pub fn set_image_as_bytes(
        &mut self,
        book: &Book,
        bytes: bytes::Bytes,
    ) -> Result<(), Box<dyn std::error::Error>> {
        _ = match self.get_image_from_storage(book) {
            Ok(Some(_)) => {
                let conn = rusqlite::Connection::open(self.storage_path.join(Self::STORAGE_FILE))?;
                conn.execute(
                    "UPDATE thumbnails SET image_data = ?2 WHERE book_url = ?1;",
                    (book.url.clone(), bytes.to_vec()),
                )
            }
            _ => {
                let conn = rusqlite::Connection::open(self.storage_path.join(Self::STORAGE_FILE))?;
                conn.execute(
                    "INSERT INTO thumbnails (book_url, image_data) values (?1, ?2)",
                    (book.url.clone(), bytes.to_vec()),
                )
            }
        }?;

        _ = self.book_covers.insert(book.url.clone(), bytes);

        Ok(())
    }

    pub fn set_image_as_bytes_to_cache(&mut self, book: &Book, bytes: bytes::Bytes) {
        _ = self.book_covers.insert(book.url.clone(), bytes);
    }

    pub fn send_thumbnail_to_storage(&self, book: Book) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = match self.book_covers.get(&book.url) {
            Some(b) => b,
            None => {
                dbg!(&book);
                return Err("No book cover".into()).into();
            }
        };

        let conn = rusqlite::Connection::open(self.storage_path.join(Self::STORAGE_FILE))?;
        let _ = conn.execute(
            "INSERT INTO thumbnails (book_url, image_data) values (?1, ?2)",
            (&book.url.clone(), &bytes.to_vec()),
        )?;

        Ok(())
    }
}
