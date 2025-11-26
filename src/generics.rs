mod lifetimes;
mod traits;

use crate::generics::lifetimes::ImportantExcerpt;
use crate::generics::traits::{Hi, NewsArticle, Summary, Tweet};
use std::fmt::Display;

pub fn generics() {
    let number_list = [34, 50, 25, 100, 65];
    let char_list = ['y', 'm', 'a', 'q'];

    let result = largest_i32(&number_list);
    let result = largest_char(&char_list);

    let result = largest_i32(&number_list[1..]);
    let result = largest_char(&char_list[..2]);

    let result = largest_generic(&number_list);
    let result = largest_generic(&char_list);

    let both_integer = Point { x: 5, y: 6 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    let char_and_string = Point { x: 'c', y: "hello" };
    let integer_and_string = both_integer.mixup(char_and_string);

    println!(
        "Point = ({},{},{})",
        both_float.x(),
        both_float.y(),
        both_float.distance_from_origin(),
    );

    println!(
        "Point = ({},{})",
        integer_and_string.x(),
        integer_and_string.y(),
    );

    both_float.print();
    integer_and_string.print();

    // Compiler error: borrow of moved value: `both_integer`
    //     - function `mixup` takes ownership of the receiver `self`, which moves `both_integer`
    // println!(
    //     "Point = ({},{})",
    //     both_integer.x(),
    //     both_integer.y(),
    // );
}

pub fn traits() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let summarizable_tweet = traits::summarizable_tweet();

    println!("1 new tweet: {}", tweet.summarize());
    println!("New article available! {}", article.summarize());

    article.hi();
    tweet.hi();

    traits::breaking_news(&article);
    traits::breaking_news(&tweet);
}

// 'a is lifetime of x.as_str() and 'b is lifetime of y.as_str()
// Lifetime of l: &str is the minimum of 'a and 'b according to function `longest_string`
// pub fn lifetimes() {
//     let x = String::from("Hello");                                  // --------------+-- 'a
//                                                                     //               |
//     {                                                               //               |
//         let y = String::from(", world!");                           // ----+-- 'b    |
//         let l = lifetimes::longest_string(x.as_str(), y.as_str());  //     |         |
//                                                                     //     |         |
//         println!("The longest string is '{l}'");                    //     |         |
//     }                                                               // ----+         |
// }                                                                   // --------------+

pub fn lifetimes() {
    let x = String::from("Hello");
    let l: &str;

    {
        let y = String::from(", world!");
        l = lifetimes::longest_string(x.as_str(), y.as_str());

        println!("The longest string is '{l}'");
    }

    // println!("The longest string is '{l}'"); // Compiler error: borrowed value does not live long enough

    {
        let novel = String::from("Call me Michael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");

        let i = ImportantExcerpt {
            part: first_sentence,
        };

        // The scope here creates an instance of the ImportantExcerpt struct that holds a reference to the first sentence of the String owned by the variable novel.
        // The data in novel exists before the ImportantExcerpt instance is created.
        // In addition, novel doesn’t go out of scope until after the ImportantExcerpt goes out of scope, so the reference in the ImportantExcerpt instance is valid.
        println!("Important excerpt is: {}", i.part);
    }

    // One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the entire duration of the program. 
    // All string literals automatically have the 'static lifetime.
    let s: &'static str = "I have a static lifetime.";
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<T2, U2>(self, other: Point<T2, U2>) -> Point<T, U2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Conditionally implementing a method on a generic type depending on trait bounds (supports impl Trait syntax also)
impl<T, U> Point<T, U>
where
    T: Display,
    U: Display,
{
    fn print(&self) {
        println!("({},{})", self.x, self.y)
    }
}

impl Point<f64, f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}
