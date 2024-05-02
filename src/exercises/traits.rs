#![allow(unused)]

use std::{
    fmt::Debug,
    io::{Error, ErrorKind},
};

pub struct Tweet {
    id: String,
    username: String,
    content: String,
    tweet_type: TweetType,
}

pub enum TweetType {
    New,
    Retweet(String),
    Reply(String),
}

impl Tweet {
    pub fn new(
        id: String,
        username: String,
        content: String,
        tweet_type: TweetType,
    ) -> Result<Tweet, Error> {
        if username.len() < 1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Username field must not be empty.",
            ));
        }
        if content.len() > 280 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "Tweet content must be 280 characters or less. Got {} characters.",
                    content.len()
                ),
            ));
        }
        match tweet_type {
            TweetType::New => Ok(Tweet {
                id,
                username,
                content,
                tweet_type,
            }),
            TweetType::Retweet(str) if str.len() < 1 => Err(Error::new(
                ErrorKind::InvalidInput,
                "Retweeted user username field must not be empty.",
            )),
            TweetType::Retweet(_) => Ok(Tweet {
                id,
                username,
                content,
                tweet_type,
            }),
            TweetType::Reply(str) if str == id => Err(Error::new(
                ErrorKind::InvalidInput,
                "A reply Tweet cannot be a reply to itself.",
            )),
            TweetType::Reply(str) if str.len() < 1 => Err(Error::new(
                ErrorKind::InvalidInput,
                "A reply Tweet must be a reply to an existing Tweet.",
            )),
            TweetType::Reply(_) => Ok(Tweet {
                id,
                username,
                content,
                tweet_type,
            }),
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn tweet_type(&self) -> &TweetType {
        &self.tweet_type
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        match &self.tweet_type {
            TweetType::New => format!("@{}: \"{}\"", self.username, self.content),
            TweetType::Reply(id) => {
                format!(
                    "@{} reply to Tweet {}: \"{}\"",
                    self.username, id, self.content
                )
            }
            TweetType::Retweet(rtd_user) => format!(
                "@{} retweet from {}: \"{}\"",
                self.username, rtd_user, self.content
            ),
        }
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "\"{}\", by {} ({})",
            self.headline, self.author, self.location
        )
    }
}

pub struct Book {
    pub title: String,
    pub author: String,
    pub year: String,
}

impl Summary for Book {
    fn summarize(&self) -> String {
        format!("\"{}\" by {}", self.title, self.author)
    }

    fn tldr(&self) -> String {
        String::from("WHAT?!?!? Come on, it's a book, just read it!")
    }
}

pub trait Summary {
    // Implementation required
    fn summarize(&self) -> String;

    // Implementation not required; default implementation provided.
    fn tldr(&self) -> String {
        String::from("(read more...)")
    }
}

pub fn start() {
    // to use, must bring Summary trait into scope
    let new_tweet = Tweet::new(
        String::from("1rh2g43834hy93h3jrg"), 
        String::from("calebpitts1997"), 
        String::from("Drug cocktail for a good time: 200mg Friendship, 150 mg Fun, A key bump of Openness, and a half tab of Selflessness."), 
        TweetType::New
    ).expect("Invalid tweet :(");
    let new_reply = Tweet::new(
        String::from("2ni293453ogv34qg9"),
        String::from("AndEfficiency"),
        String::from("yoooooo that's lowkey facts 🙌"),
        TweetType::Reply(String::from("1rh2g43834hy93h3jrg")),
    )
    .expect("Invalid reply :(");
    let new_article = NewsArticle {
        headline: String::from("World's Biggest Lollipop Licked!"),
        location: String::from("Calgary, AB, Canada"),
        author: String::from("Patrick"),
        content: String::from("blah blah blah this is the article blah blah blah."),
    };
    let new_book = Book {
        title: String::from("Giant Bees vs a Tiny Me"),
        author: String::from("CalebDaComedian"),
        year: String::from("2000 A.D."),
    };

    alert(&new_tweet);
    alert(&new_reply);
    alert(&new_article);
    alert(&new_book);
    not_interested(&new_tweet);
    not_interested(&new_book);
}

pub fn alert(item: &impl Summary) {
    println!("Alert! {}", item.summarize());
}

pub fn not_interested(item: &impl Summary) {
    println!("{}", item.tldr());
}

// Forces both params to be the same type
pub fn combo<T: Summary>(item1: &T, item2: &T) {}

// Specify multiple trait bounds
pub fn notify_then_display_short(item: &(impl Summary + std::fmt::Display)) {}
pub fn notify_then_display<T: Summary + std::fmt::Display>(item: &T) {}

// use where clause for long lists of trait bounds
fn super_complex_function<T, U>(t: &T, u: &U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + Debug,
{
    return 1;
}

// in impl blocks with generic type params, can implement methods conditionally for types that implement the specified traits
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// blanket implementations: conditionally implement a trait for any type that implements another trait
trait ToSummaryString {
    fn to_summary_string(&self) -> String;
}

impl<T> ToSummaryString for T
where
    T: Summary,
{
    fn to_summary_string(&self) -> String {
        String::from("this is a dumb function!")
    }
}
