//enums
pub enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move {x: i32, y: i32},
    Write(String),
}
fn main() {
    let x: Message = Message::Move {x: 1, y: 1};
    let y: Message = Message::Write("hello world".to_string());
}
