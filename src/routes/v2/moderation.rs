use super::ApiError;
use crate::database;
use crate::models::projects::ProjectStatus;
use crate::util::auth::check_is_moderator_from_headers;
use actix_web::{delete, get, web, HttpRequest, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("moderation")
            .service(get_projects)
            .service(ban_user)
            .service(unban_user),
    );
}

#[derive(Deserialize)]
pub struct ResultCount {
    #[serde(default = "default_count")]
    pub count: i16,
}

fn default_count() -> i16 {
    100
}

#[get("projects")]
pub async fn get_projects(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    redis: web::Data<deadpool_redis::Pool>,
    count: web::Query<ResultCount>,
) -> Result<HttpResponse, ApiError> {
    check_is_moderator_from_headers(req.headers(), &**pool).await?;

    use futures::stream::TryStreamExt;

    let project_ids = sqlx::query!(
        "
        SELECT id FROM mods
        WHERE status = $1
        ORDER BY queued ASC
        LIMIT $2;
        ",
        ProjectStatus::Processing.as_str(),
        count.count as i64
    )
    .fetch_many(&**pool)
    .try_filter_map(|e| async {
        Ok(e.right().map(|m| database::models::ProjectId(m.id)))
    })
    .try_collect::<Vec<database::models::ProjectId>>()
    .await?;

    let projects: Vec<_> =
        database::Project::get_many_ids(&project_ids, &**pool, &redis)
            .await?
            .into_iter()
            .map(crate::models::projects::Project::from)
            .collect();

    Ok(HttpResponse::Ok().json(projects))
}

#[derive(Deserialize)]
pub struct BanUser {
    pub id: i64,
}

#[get("ban")]
pub async fn ban_user(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    id: web::Query<BanUser>,
) -> Result<HttpResponse, ApiError> {
    check_is_moderator_from_headers(req.headers(), &**pool).await?;

    sqlx::query!("INSERT INTO banned_users (github_id) VALUES ($1);", id.id)
        .execute(&**pool)
        .await?;

    Ok(HttpResponse::NoContent().body(""))
}

#[delete("ban")]
pub async fn unban_user(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    id: web::Query<BanUser>,
) -> Result<HttpResponse, ApiError> {
    check_is_moderator_from_headers(req.headers(), &**pool).await?;

    sqlx::query!("DELETE FROM banned_users WHERE github_id = $1;", id.id)
        .execute(&**pool)
        .await?;

    Ok(HttpResponse::NoContent().body(""))
}
