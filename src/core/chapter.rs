use async_trait::async_trait;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Chapter {
    pub number: Option<u32>,
    pub name: Option<String>,
    pub url: Option<String>,
}

impl Chapter {
    pub fn new(number: Option<u32>, name: Option<String>, url: Option<String>) -> Chapter {
        Chapter { number, name, url }
    }
}
