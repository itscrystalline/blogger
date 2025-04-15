use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::{ParseError, Url};

#[derive(Default)]
pub struct ProjectBuilder {
    title: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
    links: Vec<Link>,
}
#[derive(Default, Debug)]
pub struct LinkBuilder {
    name: Option<String>,
    url: Option<String>,
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ProjectBuilderError {
    #[error("Missing Title")]
    NoTitle,
    #[error("Missing Description")]
    NoDescription,
    #[error("Missing at least one Tag")]
    NoTags,
    #[error("Missing at least one Link")]
    NoLinks,
}
#[derive(Error, Debug, PartialEq, Eq)]
pub enum LinkBuilderError {
    #[error("Missing Link name")]
    NoName,
    #[error("Missing URL")]
    NoUrl,
    #[error("Invalid URL {given} (error: {source:?})")]
    InvalidUrl { given: String, source: ParseError },
}

impl ProjectBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(self, title: &str) -> Self {
        todo!()
    }
    pub fn description(self, description: &str) -> Self {
        todo!()
    }
    pub fn add_tag(self, tag: &str) -> Self {
        todo!()
    }
    pub fn add_link(self, link: Link) -> Self {
        todo!()
    }

    pub fn bulid(self) -> Result<Project, (Self, ProjectBuilderError)> {
        match self {
            Self {
                title,
                description,
                tags,
                links,
            } if title.is_some()
                && description.is_some()
                && !tags.is_empty()
                && !links.is_empty() =>
            {
                Ok(Project {
                    title: title.unwrap(),
                    description: description.unwrap(),
                    tags,
                    links,
                })
            }
            Self { ref title, .. } if title.is_none() => Err((self, ProjectBuilderError::NoTitle)),
            Self {
                ref description, ..
            } if description.is_none() => Err((self, ProjectBuilderError::NoTitle)),
            Self { ref tags, .. } if tags.is_empty() => Err((self, ProjectBuilderError::NoTitle)),
            Self { ref links, .. } if links.is_empty() => Err((self, ProjectBuilderError::NoTitle)),
            _ => unreachable!(),
        }
    }
}

impl LinkBuilder {
    fn new() -> Self {
        Self::default()
    }
    fn sample() -> Link {
        Link {
            name: "Example".to_string(),
            url: Url::parse("https://example.com").unwrap(),
        }
    }

    fn name(self, name: &str) -> Self {
        todo!()
    }
    fn url(self, url: &str) -> Self {
        todo!()
    }

    fn bulid(self) -> Result<Link, (Self, LinkBuilderError)> {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    title: String,
    description: String,
    tags: Vec<String>,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    name: String,
    url: Url,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_link() {
        let link = LinkBuilder::new()
            .name("Google")
            .url("https://google.com")
            .bulid();
        assert!(link.is_ok());
        let res = link.unwrap();
        assert_eq!(res.name, "Google");
        assert_eq!(res.url, Url::parse("https://google.com").unwrap());
    }
    #[test]
    fn no_name_link() {
        let link = LinkBuilder::new().url("https://google.com").bulid();
        assert!(link.is_err());

        assert!(matches!(link, Err((_, LinkBuilderError::NoName))));
    }
    #[test]
    fn empty_name_link() {
        let link = LinkBuilder::new()
            .name("")
            .url("https://google.com")
            .bulid();
        assert!(link.is_err());

        assert!(matches!(link, Err((_, LinkBuilderError::NoName))));
    }
    #[test]
    fn invalid_url_link() {
        let link = LinkBuilder::new().name("Invalid").url("://hello").bulid();
        assert!(link.is_err());

        assert!(
            matches!(link, Err((_, LinkBuilderError::InvalidUrl { given, .. })) if given == "://hello")
        )
    }

    #[test]
    fn new_project() {
        let result = ProjectBuilder::new()
            .title("A")
            .description("hello!")
            .add_tag("test")
            .add_link(LinkBuilder::sample())
            .bulid();
        assert!(result.is_ok());
    }
    #[test]
    fn no_title_project() {
        let proj = ProjectBuilder::new().bulid();
        let proj_with_others = ProjectBuilder::new()
            .description("hi")
            .add_tag("a")
            .add_link(LinkBuilder::sample())
            .bulid();

        assert!(matches!(proj, Err((_, ProjectBuilderError::NoTitle))));
        assert!(matches!(
            proj_with_others,
            Err((_, ProjectBuilderError::NoTitle))
        ));
    }
    #[test]
    fn no_description_project() {
        let proj = ProjectBuilder::new().title("hi").bulid();
        let proj_with_others = ProjectBuilder::new()
            .title("hi")
            .add_link(LinkBuilder::sample())
            .add_tag("meow")
            .bulid();

        assert!(matches!(proj, Err((_, ProjectBuilderError::NoDescription))));
        assert!(matches!(
            proj_with_others,
            Err((_, ProjectBuilderError::NoDescription))
        ));
    }
    #[test]
    fn no_links_project() {
        let proj = ProjectBuilder::new()
            .title("hi")
            .description("hello")
            .bulid();
        let proj_with_others = ProjectBuilder::new()
            .title("hi")
            .description("hello")
            .add_tag("meow")
            .bulid();

        assert!(matches!(proj, Err((_, ProjectBuilderError::NoLinks))));
        assert!(matches!(
            proj_with_others,
            Err((_, ProjectBuilderError::NoLinks))
        ));
    }
    #[test]
    fn no_tags_project() {
        let proj = ProjectBuilder::new()
            .title("hi")
            .description("hello")
            .add_link(LinkBuilder::sample())
            .bulid();

        assert!(matches!(proj, Err((_, ProjectBuilderError::NoTags))));
    }
}
