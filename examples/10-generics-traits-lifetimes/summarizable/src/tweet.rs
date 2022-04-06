#[derive(Debug)]
pub struct Tweet {
    pub content: String,
    pub handle: String,
}

impl Tweet {
    pub fn summarize(&self) -> String {
        format!("{} by @{}", self.content, self.handle)
    }
}
