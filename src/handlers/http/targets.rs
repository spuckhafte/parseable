use actix_web::{
    HttpRequest, Responder,
    web::{self, Json, Path},
};
use itertools::Itertools;
use ulid::Ulid;

use crate::alerts::{
    AlertError,
    target::{TARGETS, Target},
};

use crate::{create_target, delete_target, get_target_by_id, list_targets, update_target};

create_target! {
/// Create target
///
/// Creates a new notification target for alerts.
pub async fn post(
    _req: HttpRequest,
    Json(target): Json<Target>,
) -> Result<impl Responder, AlertError> {
    // should check for duplicacy and liveness (??)
    // add to the map
    TARGETS.update(target.clone()).await?;

    // Ok(web::Json(target.mask()))
    Ok(web::Json(target))
}
}

list_targets! {
/// List targets
///
/// Returns all configured notification targets.
pub async fn list(_req: HttpRequest) -> Result<impl Responder, AlertError> {
    // add to the map
    let list = TARGETS
        .list()
        .await?
        .into_iter()
        // .map(|t| t.mask())
        .collect_vec();

    Ok(web::Json(list))
}
}

get_target_by_id! {
/// Get target
///
/// Retrieves a specific target by ID.
pub async fn get(_req: HttpRequest, target_id: Path<Ulid>) -> Result<impl Responder, AlertError> {
    let target_id = target_id.into_inner();

    let target = TARGETS.get_target_by_id(&target_id).await?;

    // Ok(web::Json(target.mask()))
    Ok(web::Json(target))
}
}

update_target! {
/// Update target
///
/// Updates an existing target's configuration.
pub async fn update(
    _req: HttpRequest,
    target_id: Path<Ulid>,
    Json(mut target): Json<Target>,
) -> Result<impl Responder, AlertError> {
    let target_id = target_id.into_inner();

    // if target_id does not exist, error
    let old_target = TARGETS.get_target_by_id(&target_id).await?;

    // do not allow modifying name
    if old_target.name != target.name {
        return Err(AlertError::InvalidTargetModification(
            "Can't modify target name".to_string(),
        ));
    }

    // esnure that the supplied target id is assigned to the target config
    target.id = target_id;

    // should check for duplicacy and liveness (??)
    // add to the map
    TARGETS.update(target.clone()).await?;

    // Ok(web::Json(target.mask()))
    Ok(web::Json(target))
}
}

delete_target! {
/// Delete target
///
/// Permanently deletes a notification target.
pub async fn delete(
    _req: HttpRequest,
    target_id: Path<Ulid>,
) -> Result<impl Responder, AlertError> {
    let target_id = target_id.into_inner();

    let target = TARGETS.delete(&target_id).await?;

    // Ok(web::Json(target.mask()))
    Ok(web::Json(target))
}
}
