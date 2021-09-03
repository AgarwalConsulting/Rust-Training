// #[derive(Debug)]
// struct Pair<T>(T, T);
use std::fmt::{Display, Formatter, Error};

trait Summary {
    fn new<T>() -> T;

    fn summary(&self) -> String {
        String::from("Click to read more...")
    }
}

#[derive(Debug)]
struct NewsArticle {
    title: String,
    body: String,
    author: String,
    location: String,
    tags: Vec<String>
}

impl Summary for NewsArticle {
    fn new() -> Self {
        NewsArticle{
            title: String::from(""),
            body: String::from(""),
            author: String::from(""),
            location: String::from(""),
            tags: vec!(),
        }
    }

    fn summary(&self) -> String {
        format!("{}, by {}, {}", self.title, self.author, self.location)
    }
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn new() -> Self {
        Tweet{username: String::from(""), content: String::from("")}
    }

    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// #[derive(Debug)]
// struct Pair<A>(A, A) where A: Summary;
// struct Pair<A, B>(A, B) where A: Summary, B: Summary;
struct Pair(Box<dyn Summary>, Box<dyn Summary>);

impl Display for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(&format!("{} : {}", self.0.summary(), self.1.summary()))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let news_article = NewsArticle{
        title: String::from("Neeraj wins Gold"),
        body: String::from("Lorem ipsum..."),
        author: String::from("Gaurav"),
        location: String::from("Chennai"),
        tags: vec!(String::from("sports")),
    };

    let tweet1 = Tweet{username: String::from("algogrit"), content: String::from("Teaching rust... It's awesome!")};

    let tweet2 = Tweet{username: String::from("algogrit"), content: String::from("Generics are confusing!")};

    let vec: Vec<Box<dyn Summary>> = vec!(Box::new(news_article), Box::new(tweet1), Box::new(tweet2));

    println!("Listing all summaries...");
    for el in vec {
        println!("\t{}", el.summary())
    }

    let news_article_1 = NewsArticle{
        title: String::from("More than 50% are vacinated"),
        body: String::from("Lorem ipsum..."),
        author: String::from("Gaurav"),
        location: String::from("Chennai"),
        tags: vec!(String::from("health"), String::from("world")),
    };

    let tweet3 = Tweet{username: String::from("algogrit"), content: String::from("Hopefully generics are also easy to deal with!")};

    // let p = Pair(tweet1, tweet2);
    let p = Pair(Box::new(news_article_1), Box::new(tweet3));

    println!("{}", p);

    println!("{:?}", p.0.summary());

    Ok(())
}
