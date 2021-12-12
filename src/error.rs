pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error{
    StrConversation,
    IO(std::io::Error),
}