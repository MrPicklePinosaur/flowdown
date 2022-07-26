use thiserror::Error;

pub type Result<T> = std::result::Result<T, FlowdownError>;

#[derive(Error, Debug)]
pub enum FlowdownError {
    #[error("undefined bookmarks: {0:?}")]
    UndefinedBookmark(Vec<String>),
    #[error("bookmark defined more than once {0}")]
    BookmarkAlreadyDefined(String),
    #[error("dialog defined more than once {0}")]
    DialogAlreadyDefined(String),
    #[error("cannot read source code file {0}")]
    CannotReadCodeFile(String),
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("did not provide input flowdown file")]
    MissingInputFile,
    #[error("cannot read source flowdown file {0}")]
    CannotReadFlowdownFile(String),
}
