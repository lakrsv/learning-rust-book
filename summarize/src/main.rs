use summarize::Summary;
use summarize::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("Lars"),
        content: String::from("Of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}
