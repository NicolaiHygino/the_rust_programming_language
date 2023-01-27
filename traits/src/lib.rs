use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{})", self.headline)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_trait_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_more_params(item1: &impl Summary, item2: &impl Summary) {
    println!(
        "Breaking news! {}, {}",
        item1.summarize(),
        item2.summarize()
    );
}

pub fn notify_trait_bound_force_same_trait<T: Summary>(item1: &T, item2: &T) {
    println!(
        "Breaking news! {}, {}",
        item1.summarize(),
        item2.summarize()
    );
}

pub fn notify_multiple_trait_bounds_sugar(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_multiple_trait_bounds<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    50
}

pub fn retuns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("nicolai_lindo"),
        content: String::from("he is so handsome"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The larges member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
