use super::my_traits::Summarizable;

#[derive(Debug)]
pub struct NewsArticle {
    pub title: String,
    pub content: String,
    pub author: String,
}

impl NewsArticle {
    pub fn summarize(&self) -> String {
        format!("\"{}\" written by {}", self.title, self.author)
    }
}

impl Summarizable for NewsArticle {
    fn summarize(&self) -> String {
        self.summarize()
        // format!("\"{}\" written by {}", self.title, self.author)
    }
}
