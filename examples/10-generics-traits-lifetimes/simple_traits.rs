trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for i32 {
    fn summarize(&self) -> String {
        format!("{:?}", self)
    }
}

struct NewsArticle {
    heading: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.heading, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    retweet_count: i32,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} - @{}", self.content, self.username)
    }
}

#[derive(Debug)]
struct Articles<T: Summary>(Vec<T>);

impl<T: Summary> Articles<T> {
    fn summarize_all(&self) {
        for article in &self.0 {
            println!("{}", article.summarize());
        }
    }
}

fn main() {
    let mut articles = Articles(vec!());
    let mut tweets = Articles(vec!());
    // let articles: Articles<NewsArticle> = Articles(vec!());
    // let articles: Vec<T: Summary> = vec!(); <- Not possible in Rust!
    // let mut articles: Articles<T: Summary> = Articles(vec!());

    articles.0.push(NewsArticle{
        heading: String::from("Some clickbaity title"),
        location: String::from("India"),
        author: String::from("John Doe"),
        content: String::from("..."),
    });

    tweets.0.push(Tweet{
        content: String::from("Silly little tweet"),
        username: String::from("algogrit"),
        retweet_count: 1000,
    });

    articles.summarize_all();
    // for article in articles.0 {
    //     println!("{}", article.summarize());
    // }

    // for tweet in tweets.0 {
    //     println!("{}", tweet.summarize());
    // }
    tweets.summarize_all();

    let mut numbers = Articles(vec!());

    numbers.0.push(1);
    numbers.0.push(2);
    numbers.0.push(3);
    numbers.0.push(5);
    numbers.0.push(7);

    println!("{:?}", numbers);

    numbers.summarize_all();
}
