use itertools::Itertools;
use url::Url;

use crate::{Link, LinkBuilderError, Project, ProjectBuilderError};

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

impl ProjectBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        _ = self.title.replace(title.to_string());
        self
    }
    pub fn description(&mut self, description: &str) -> &mut Self {
        _ = self.description.replace(description.to_string());
        self
    }
    pub fn add_tag(&mut self, tag: &str) -> &mut Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn add_link(&mut self, link: Link) -> &mut Self {
        self.links.push(link);
        self
    }
    pub fn remove_tag(&mut self, tag_name: &str) -> &mut Self {
        self.tags.retain(|tag| tag != tag_name);
        self
    }
    pub fn remove_link(&mut self, link_name: &str) -> &mut Self {
        self.links.retain(|Link { name, .. }| name != link_name);
        self
    }

    pub fn bulid(&self) -> Result<Project, ProjectBuilderError> {
        let Self {
            title,
            description,
            tags,
            links,
        } = self;
        if title.is_some() && description.is_some() && !tags.is_empty() && !links.is_empty() {
            Ok(Project {
                title: title.as_ref().unwrap().clone(),
                description: description.as_ref().unwrap().clone(),
                tags: tags.clone(),
                links: links.clone(),
            })
        } else if title.is_none() {
            Err(ProjectBuilderError::Title)
        } else if description.is_none() {
            Err(ProjectBuilderError::Description)
        } else if tags.is_empty() {
            Err(ProjectBuilderError::Tags)
        } else if links.is_empty() {
            Err(ProjectBuilderError::Links)
        } else {
            unreachable!()
        }
    }
}

impl LinkBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn sample() -> Link {
        Link {
            name: "Example".to_string(),
            url: Url::parse("https://example.com").unwrap(),
        }
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        _ = self.name.replace(name.to_string());
        self
    }
    pub fn url(&mut self, url: Url) -> &mut Self {
        _ = self.url.replace(url);
        self
    }

    pub fn bulid(&self) -> Result<Link, LinkBuilderError> {
        match self {
            LinkBuilder {
                name: Some(name),
                url: Some(url),
            } if !name.is_empty() => Ok(Link {
                name: name.clone(),
                url: url.clone(),
            }),
            LinkBuilder {
                name: Some(name), ..
            } if name.is_empty() => Err(LinkBuilderError::Name),
            LinkBuilder { name: None, .. } => Err(LinkBuilderError::Name),
            LinkBuilder { url: None, .. } => Err(LinkBuilderError::Url),
            _ => unreachable!(),
        }
    }
}
pub trait Edit {
    type Builder;
    fn edit(self) -> Self::Builder;
}

impl Edit for Project {
    type Builder = ProjectBuilder;
    fn edit(self) -> Self::Builder {
        ProjectBuilder {
            title: Some(self.title),
            description: Some(self.description),
            tags: self.tags,
            links: self.links,
        }
    }
}

impl Edit for Link {
    type Builder = LinkBuilder;
    fn edit(self) -> Self::Builder {
        LinkBuilder {
            name: Some(self.name),
            url: Some(self.url),
        }
    }
}
