use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ProjectBuilderError {
    #[error("Missing Title")]
    Title = 1,
    #[error("Missing Description")]
    Description = 2,
    #[error("Missing Cover")]
    Cover = 4,
    #[error("Missing at least one Tag")]
    Tags = 8,
    #[error("Missing at least one Link")]
    Links = 16,
    #[error("Duplicate Tags")]
    DuplicateTags = 32,
    #[error("Duplicate Links")]
    DuplicateLinks = 64,
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum LinkBuilderError {
    #[error("Missing Link name")]
    Name = 1,
    #[error("Missing URL")]
    Url = 2,
}
