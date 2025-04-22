use crate::errors::*;
use serde::{Deserialize, Serialize};
use url::Url;

mod builders;
mod errors;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    title: String,
    description: String,
    tags: Vec<String>,
    links: Vec<Link>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Link {
    name: String,
    url: Url,
}

#[cfg(test)]
mod tests {
    use builders::*;

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
    fn edit_link() {
        let link = LinkBuilder::new()
            .name("Google")
            .url(Url::parse("https://google.com").unwrap())
            .bulid();
        assert!(link.is_ok());
        let res = link.unwrap();
        assert_eq!(res.name, "Google");
        assert_eq!(res.url, Url::parse("https://google.com").unwrap());
        let mut new_link_builder = res.edit();
        new_link_builder
            .name("Youtube")
            .url(Url::parse("https://youtube.com").unwrap());
        let res = new_link_builder.bulid();
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.name, "Youtube");
        assert_eq!(res.url, Url::parse("https://youtube.com").unwrap());
    }
    #[test]
    fn no_name_link() {
        let link = LinkBuilder::new()
            .url(Url::parse("https://google.com").unwrap())
            .bulid();
        assert!(link.is_err());

        assert!(matches!(link, Err(LinkBuilderError::Name)));
    }
    #[test]
    fn empty_name_link() {
        let link = LinkBuilder::new()
            .name("")
            .url(Url::parse("https://google.com").unwrap())
            .bulid();
        assert!(link.is_err());

        assert!(matches!(link, Err(LinkBuilderError::Name)));
    }
    #[test]
    fn no_url_link() {
        let link = LinkBuilder::new().name("empty").bulid();
        assert!(link.is_err());

        assert!(matches!(link, Err(LinkBuilderError::Url)));
    }
    #[test]
    fn rebuild_invalid_link() {
        let mut link_builder = LinkBuilder::new();
        let link = link_builder.name("empty").bulid();
        assert!(link.is_err());
        assert!(matches!(link, Err(LinkBuilderError::Url)));

        link_builder.url(Url::parse("https://example.com").unwrap());
        let link = link_builder.bulid();
        assert!(link.is_ok());
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
        let project = result.unwrap();
        assert_eq!(project.title, String::from("A"));
        assert_eq!(project.description, String::from("hello!"));
        assert_eq!(project.tags[0], String::from("test"));
        assert_eq!(project.links[0], LinkBuilder::sample());
    }
    #[test]
    fn edit_project() {
        let project_result = ProjectBuilder::new()
            .title("hiya!")
            .description("lorem ipsum dolor sit amet")
            .add_tag("Rust")
            .add_link(LinkBuilder::sample())
            .bulid();
        assert!(project_result.is_ok());
        let project = project_result.unwrap();
        assert_eq!(project.title, String::from("hiya!"));
        assert_eq!(
            project.description,
            String::from("lorem ipsum dolor sit amet")
        );
        assert_eq!(project.tags[0], String::from("Rust"));
        assert_eq!(project.links[0], LinkBuilder::sample());
        let mut project_edited = project.edit();
        project_edited.add_tag("Java");
        project_edited.title("meow :3c");
        let project_edited_result = project_edited.bulid();
        assert!(project_edited_result.is_ok());

        let new_project = project_edited_result.unwrap();
        assert_eq!(new_project.title, String::from("meow :3c"));
        assert_eq!(new_project.tags[0], String::from("Rust"));
        assert_eq!(new_project.tags[1], String::from("Java"));
    }
    #[test]
    fn delete_tag_from_project() {
        let mut project_builder = ProjectBuilder::new();
        project_builder
            .title("hiya!")
            .description("lorem ipsum dolor sit amet")
            .add_tag("Rust")
            .add_tag("Java")
            .add_tag("C#")
            .add_tag("C")
            .add_link(LinkBuilder::sample());
        let project_result = project_builder.bulid();
        assert!(project_result.is_ok());
        let project = project_result.unwrap();
        assert_eq!(project.tags[0], String::from("Rust"));
        assert_eq!(project.tags[1], String::from("Java"));
        assert_eq!(project.tags[2], String::from("C#"));
        assert_eq!(project.tags[3], String::from("C"));

        let mut project_builder = project.edit();
        project_builder.remove_tag("Java");
        project_builder.remove_tag("C#");
        let project_result = project_builder.bulid();
        assert!(project_result.is_ok());
        let project = project_result.unwrap();
        assert_eq!(project.tags[0], String::from("Rust"));
        assert_eq!(project.tags[1], String::from("C"));
    }
    #[test]
    fn delete_link_from_project() {
        let mut project_builder = ProjectBuilder::new();
        project_builder
            .title("hiya!")
            .description("lorem ipsum dolor sit amet")
            .add_tag("Rust")
            .add_link(LinkBuilder::sample())
            .add_link(
                LinkBuilder::new()
                    .name("google")
                    .url(Url::parse("https://google.com").unwrap())
                    .bulid()
                    .unwrap(),
            );
        let project_result = project_builder.bulid();
        assert!(project_result.is_ok());
        let project = project_result.unwrap();
        assert_eq!(project.links[0].name, String::from("Example"));
        assert_eq!(project.links[1].name, String::from("google"));

        let mut project_builder = project.edit();
        project_builder.remove_link("Example");
        let project_result = project_builder.bulid();
        assert!(project_result.is_ok());
        let project = project_result.unwrap();
        assert_eq!(project.links[0].name, String::from("google"));
    }
    #[test]
    fn no_title_project() {
        let proj = ProjectBuilder::new().bulid();
        let proj_with_others = ProjectBuilder::new()
            .description("hi")
            .add_tag("a")
            .add_link(LinkBuilder::sample())
            .bulid();

        assert!(matches!(proj, Err(ProjectBuilderError::Title)));
        assert!(matches!(proj_with_others, Err(ProjectBuilderError::Title)));
    }
    #[test]
    fn no_description_project() {
        let proj = ProjectBuilder::new().title("hi").bulid();
        let proj_with_others = ProjectBuilder::new()
            .title("hi")
            .add_link(LinkBuilder::sample())
            .add_tag("meow")
            .bulid();

        assert!(matches!(proj, Err(ProjectBuilderError::Description)));
        assert!(matches!(
            proj_with_others,
            Err(ProjectBuilderError::Description)
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

        assert!(matches!(proj, Err(ProjectBuilderError::Tags)));
        assert!(matches!(proj_with_others, Err(ProjectBuilderError::Tags)));
    }
    #[test]
    fn no_links_project() {
        let proj = ProjectBuilder::new()
            .title("hi")
            .description("hello")
            .add_tag("meow")
            .bulid();

        assert!(matches!(proj, Err(ProjectBuilderError::Links)));
    }
    #[test]
    fn rebuild_invalid_project() {
        let mut proj = ProjectBuilder::new();
        proj.title("hi").description("hello").add_tag("meow");

        let try_build = proj.bulid();
        assert!(matches!(try_build, Err(ProjectBuilderError::Links)));

        proj.add_link(LinkBuilder::sample());
        let build_now = proj.bulid();
        assert!(build_now.is_ok());
    }
}
