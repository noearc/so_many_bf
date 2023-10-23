#[derive(Debug, PartialEq)]
pub enum Token {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Print,
    Read,
    OpenLoop,
    CloseLoop,
    Comment(String),
}
