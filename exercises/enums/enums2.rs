// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!

// DONE

#[derive(Debug)]
enum Message {
    Move {
        x: u32,
        y: u32,
    },
    Echo(String),
    ChangeColor(u8,u8,u8), //assumed is RGB
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
