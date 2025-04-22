use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ProjectBuilderError {
    #[error("Missing Title")]
    Title = 1,
    #[error("Missing Description")]
    Description = 2,
    #[error("Missing at least one Tag")]
    Tags = 4,
    #[error("Missing at least one Link")]
    Links = 8,
    #[error("Duplicate Tags")]
    DuplicateTags = 16,
    #[error("Duplicate Links")]
    DuplicateLinks = 32,
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum LinkBuilderError {
    #[error("Missing Link name")]
    Name = 1,
    #[error("Missing URL")]
    Url = 2,
}
