use std::fmt::{Display, Formatter, Result};

// Generic type parameter that is trait bound on 2 traits
pub fn breaking_news<T: Summary + Display>(news: &T) {
    println!("Breaking news! {}", news.summarize());
}

// Return a trait instead of the type
pub fn summarizable_tweet() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// Generic type parameter that is trait bound on 2 traits with more scalable where clause
fn breaking_news_where_clause<T>(news: &T)
where
    T: Summary + Display,
{
    println!("Breaking news! {}", news.summarize());
}

// The impl Trait syntax is just syntax suger for a longer form known as a trait bound
fn breaking_news_impl_trait(news: &(impl Summary + Display)) {
    println!("Breaking news! {}", news.summarize());
}

// The impl Trait syntax is just syntax suger for a longer form known as a trait bound
fn breaking_news_impl_1_trait(news: &impl Summary) {
    println!("Breaking news! {}", news.summarize());
}

// A type’s behavior consists of the methods we can call on that type.
// Different types share the same behavior if we can call the same methods on all of those types.
// Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.
pub trait Summary {
    // Signature declaration without default implementation
    fn summarize_author(&self) -> String;

    // Signature with default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub trait Hi {
    // Signature declaration without default implementation
    fn hi(&self);
}

// Conditionally implement a trait `Hi` for any type that implements another trait `Summary`.
// Implementations of a trait `Hi` on any type that satisfies the trait bounds `Summary` are called blanket implementations.
// Blanket implementation of trait `Hi` for type T that is trait bound on trait `Summary`
impl<T> Hi for T
where
    T: Summary,
{
    fn hi(&self) {
        println!("Hi {}!", Summary::summarize_author(self));
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Use trait's default methods implementation
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

// Implement trait for type
impl Display for NewsArticle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.headline, self.location, self.author, self.content
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Implement trait for type
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // Override trait's default methods implementation
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.username, self.content, self.reply, self.retweet
        )
    }
}
