fn main() {
    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 5.0, y: 10.0 };
    let point = Point { x: 12, y: 20 };
    println!("{}", point.summarize());
    // x:12,y:20
    let tweet = Tweet {
        username: 12.to_string(),
        content: 20.to_string(),
    };
    println!("{}", tweet.summarize());
    // 12 and 20
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub struct Tweet {
    pub username: String,
    pub content: String,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} and {}", self.username, self.content)
    }
}

impl Summary for Point {
    fn summarize(&self) -> String {
        format!("x:{},y:{}", self.x, self.y)
    }
}
