use rocket_db_pools::{
    Connection, Database,
    sqlx::{self, Result, Row, mysql::MySqlRow},
};
use url::Url;

use crate::{
    Link, Project, Projects,
    builders::{LinkBuilder, ProjectBuilder},
};

#[derive(Database)]
#[database("blogger")]
pub struct BloggerDatabase(sqlx::MySqlPool);

impl Projects {
    pub async fn get(mut db: Connection<BloggerDatabase>) -> Result<Projects> {
        let mut projects_incomplete = sqlx::query("SELECT * FROM projects")
            .map(|row: MySqlRow| {
                let mut proj = ProjectBuilder::new();
                proj.title(row.get("title"))
                    .description(row.get("description"));
                if let Some(cover) = row.get::<Option<String>, _>("cover") {
                    if let Ok(cover_link) = Url::parse(&cover) {
                        proj.cover(cover_link);
                    }
                }
                (row.get::<u32, _>("id"), proj)
            })
            .fetch_all(&mut **db)
            .await?;
        let tags = sqlx::query("SELECT project_id, tag FROM project_tags")
            .map(|row: MySqlRow| (row.get::<u32, _>("project_id"), row.get::<String, _>("tag")))
            .fetch_all(&mut **db)
            .await?;
        let links = sqlx::query("SELECT project_id, name, link FROM project_tags")
            .map(|row: MySqlRow| {
                let proj_id: u32 = row.get("project_id");
                let mut link_builder = LinkBuilder::new();
                link_builder.name(row.get("name"));
                if let Some(link) = row.get::<Option<String>, _>("link") {
                    if let Ok(link) = Url::parse(&link) {
                        link_builder.url(link);
                    }
                }
                (proj_id, link_builder)
            })
            .fetch_all(&mut **db)
            .await?;
        todo!()
    }
}
