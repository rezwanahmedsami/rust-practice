
pub struct NewsArticle{
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summaraize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summaraize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summaraize(&self) -> String;
}

pub fn run(){
    let tweet = Tweet {
        username: String::from("@rezwanahmodsami"),
        content: String::from("This is a good day"),
        reply: true,
        retweet: false
    };

    let newsarticle = NewsArticle {
        author: String::from("Rezwan ahmed"),
        headline: String::from("Bangladesh facing blackout"),
        content: String::from("bangladesh is in big problem of loadshedding.")
    };

    println!("Tweet summarize: {}", tweet.summaraize());
    println!("Article summarize: {}", newsarticle.summaraize());
}