use thiserror::Error;

pub type Result<T> = std::result::Result<T, FlowdownError>;

#[derive(Error, Debug)]
pub enum FlowdownError {
    #[error("undefined bookmarks: {0:?}")]
    UndefinedBookmark(Vec<String>),
}
