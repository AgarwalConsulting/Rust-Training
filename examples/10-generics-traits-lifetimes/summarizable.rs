trait Summarizable {
    fn summary(&self) -> String {
        String::from("Click to read more...")
    }
}

struct NewsArticle {
    title: String,
    body: String,
    author: String,
    location: String,
    tags: Vec<String>
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {}, {}", self.title, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// fn summarize(art: NewsArticle) {
//     println!("{}", art.summary());
// }

fn summarize<T: Summarizable>(art: T) {
    println!("{}", art.summary());
}

// fn summarize(art: impl Summarizable) {
//     println!("{}", art.summary());
// }

fn main() {
    let news_article = NewsArticle{
        title: String::from("Neeraj wins Gold"),
        body: String::from("Lorem ipsum..."),
        author: String::from("Gaurav"),
        location: String::from("Chennai"),
        tags: vec!(String::from("sports")),
    };

    summarize::<NewsArticle>(news_article);

    let tweet = Tweet{username: String::from("algogrit"), content: String::from("Teaching rust... It's awesome!")};

    summarize(tweet);
}
