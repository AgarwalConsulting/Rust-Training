use std::fmt::Display;

trait Summary {
    fn summarize(&self) -> String;
}

// trait Summary {
//     type T = Self;

//     fn summarize<T: Display>(&self) -> String {
//         format!("{}", self)
//     }
// }


// struct List<T: Summary> {
//     items: Vec<T>
// }

struct Point<T: Display>(T, T);

impl<T: Display> Summary for Point<T> {
    fn summarize(&self) -> String {
        format!("{}, {}", self.0, self.1)
    }
}

struct List {
    items: Vec<Box<dyn Summary>>,
}

impl List {
    fn summarize_all(&self) {
        // let item: Box<dyn Summary>;
        for item in &self.items {
            println!("{}", item.summarize());
        }
    }
}

impl Summary for i32 {
    fn summarize(&self) -> String {
        format!("{}", self)
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

fn main() {
    let news_article = NewsArticle{
        heading: String::from("Some clickbaity title"),
        location: String::from("India"),
        author: String::from("John Doe"),
        content: String::from("..."),
    };

    let tweet = Tweet{
        content: String::from("Silly little tweet"),
        username: String::from("algogrit"),
        retweet_count: 1000,
    };

    let mut l = List{items: vec!()};

    l.items.push(Box::new(news_article));

    l.items.push(Box::new(tweet));

    l.items.push(Box::new(Point(10, 12)));

    // println!("{:?}", l);
    l.summarize_all();
}
