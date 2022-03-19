fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 151 };
    match msg {
        Message::Hello { id: id_var @ 3..=7 } => {
            println!("Found an id in range: {}", id_var)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id :{}", id)
        }
    }
}
