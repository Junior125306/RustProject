use std::vec;

use Object::Draw;
use Object::{Button, Screen};

struct SelectBox {
    witdh: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        //绘制
    }
}

fn main() {
    let screen = Screen {
        compoents: vec![
            Box::new(SelectBox {
                witdh: 75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("maybe"),
                    String::from("no"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
}
