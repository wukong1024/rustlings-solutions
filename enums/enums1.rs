// enums1.rs
// No hints this time! ;)

#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo(String),
    Move { x: u32, y: u32 },
    ChangeColor(u32, u32, u32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("123")));
    println!("{:?}", Message::Move { x: 12, y: 12 });
    println!("{:?}", Message::ChangeColor(100, 100, 100));
}
