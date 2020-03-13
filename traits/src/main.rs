pub trait Summary {
    fn summarize(&self) -> String;
    fn abreviated_summary(&self) -> &str {
        &self.summarize()[0..4]
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
        format!("{} by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

fn main() {
    let tweet = Tweet {
        username: String::from("TheGreatYawner"),
        content: "I saw an okay movie yesterday. Have to say i'm alright with it all.".to_string(),
        reply: false,
        retweet: false,
    };

    let nyt = NewsArticle {
        author: "Jacob Feldman".to_string(),
        headline: "The great programmer, myself".to_string(),
        location: "NYC".to_string(),
        content: "Jacob is a great programmer, or so i'm told.".to_string(),
    };

    println!("{}", nyt.summarize());
    println!("{}", nyt.abreviated_summary());
    //works because nyt has implemented the
    //summarize trait
    //Tweet.summarize(); //wont compile because tweet doesnt implement summarize
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
