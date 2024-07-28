use async_trait::async_trait;

use super::Chapter;

impl Chapter {
    pub fn new(number: Option<u32>, name: Option<String>, url: Option<String>) -> Chapter {
        Chapter { number, name, url }
    }
}
