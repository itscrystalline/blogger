use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::{ParseError, Url};

#[derive(Default, Debug, Clone)]
pub struct ProjectBuilder {
    title: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
    links: Vec<Link>,
}
#[derive(Default, Debug, Clone)]
pub struct LinkBuilder {
    name: Option<String>,
    url: Option<Url>,
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
}

impl ProjectBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
    pub fn add_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn add_link(mut self, link: Link) -> Self {
        self.links.push(link);
        self
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
            Self {
                ref title,
                ref description,
                ref tags,
                ref links,
            } => {
                if title.is_none() {
                    Err((self, ProjectBuilderError::NoTitle))
                } else if description.is_none() {
                    Err((self, ProjectBuilderError::NoDescription))
                } else if tags.is_empty() {
                    Err((self, ProjectBuilderError::NoTags))
                } else if links.is_empty() {
                    Err((self, ProjectBuilderError::NoLinks))
                } else {
                    unreachable!()
                }
            }
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

    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    fn url(mut self, url: Url) -> Self {
        self.url = Some(url);
        self
    }

    fn bulid(self) -> Result<Link, (Self, LinkBuilderError)> {
        match self {
            LinkBuilder {
                name: Some(name),
                url: Some(url),
            } if !name.is_empty() => Ok(Link { name, url }),
            LinkBuilder {
                name: Some(ref name),
                ..
            } if name.is_empty() => Err((self, LinkBuilderError::NoName)),
            LinkBuilder { name: None, .. } => Err((self, LinkBuilderError::NoName)),
            LinkBuilder { url: None, .. } => Err((self, LinkBuilderError::NoUrl)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    title: String,
    description: String,
    tags: Vec<String>,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
            .url(Url::parse("https://google.com").unwrap())
            .bulid();
        assert!(link.is_ok());
        let res = link.unwrap();
        assert_eq!(res.name, "Google");
        assert_eq!(res.url, Url::parse("https://google.com").unwrap());
    }
    #[test]
    fn no_name_link() {
        let link = LinkBuilder::new()
            .url(Url::parse("https://google.com").unwrap())
            .bulid();
        assert!(link.is_err());

        assert!(matches!(link, Err((_, LinkBuilderError::NoName))));
    }
    #[test]
    fn empty_name_link() {
        let link = LinkBuilder::new()
            .name("")
            .url(Url::parse("https://google.com").unwrap())
            .bulid();
        assert!(link.is_err());

        assert!(matches!(link, Err((_, LinkBuilderError::NoName))));
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
    fn no_tags_project() {
        let proj = ProjectBuilder::new()
            .title("hi")
            .description("hello")
            .bulid();
        let proj_with_others = ProjectBuilder::new()
            .title("hi")
            .description("hello")
            .add_link(LinkBuilder::sample())
            .bulid();

        assert!(matches!(proj, Err((_, ProjectBuilderError::NoTags))));
        assert!(matches!(
            proj_with_others,
            Err((_, ProjectBuilderError::NoTags))
        ));
    }
    #[test]
    fn no_links_project() {
        let proj = ProjectBuilder::new()
            .title("hi")
            .description("hello")
            .add_tag("meow")
            .bulid();

        assert!(matches!(proj, Err((_, ProjectBuilderError::NoLinks))));
    }
}
