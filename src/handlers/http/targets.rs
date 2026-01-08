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

/// Create target
///
/// Creates a new notification target for alerts.
#[utoipa::path(
    post,
    path = "/api/v1/targets",
    tag = "targets",
    summary = "Create target",
    description = "Creates a new notification target (webhook, Slack, email, etc.) that can be used by alerts. Automatically generates a unique ID.",
    request_body = Target,
    responses(
        (status = 200, description = "Target created successfully", body = Target),
        (status = 400, description = "Invalid target configuration")
    ),
    security(
        ("authorization" = [])
    )
)]
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

/// List targets
///
/// Returns all configured notification targets.
#[utoipa::path(
    get,
    path = "/api/v1/targets",
    tag = "targets",
    summary = "List targets",
    description = "Retrieves all notification targets configured in the system.",
    responses(
        (status = 200, description = "List of targets", body = Vec<Target>)
    ),
    security(
        ("authorization" = [])
    )
)]
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

/// Get target
///
/// Retrieves a specific target by ID.
#[utoipa::path(
    get,
    path = "/api/v1/targets/{target_id}",
    tag = "targets",
    summary = "Get target",
    description = "Returns complete configuration for the specified notification target.",
    params(
        ("target_id" = String, Path, description = "Target ID (ULID format)")
    ),
    responses(
        (status = 200, description = "Target details", body = Target),
        (status = 404, description = "Target not found")
    ),
    security(
        ("authorization" = [])
    )
)]
pub async fn get(_req: HttpRequest, target_id: Path<Ulid>) -> Result<impl Responder, AlertError> {
    let target_id = target_id.into_inner();

    let target = TARGETS.get_target_by_id(&target_id).await?;

    // Ok(web::Json(target.mask()))
    Ok(web::Json(target))
}

/// Update target
///
/// Updates an existing target's configuration.
#[utoipa::path(
    put,
    path = "/api/v1/targets/{target_id}",
    tag = "targets",
    summary = "Update target",
    description = "Modifies notification target configuration. Updates both storage and in-memory representation.",
    params(
        ("target_id" = String, Path, description = "Target ID (ULID format)")
    ),
    request_body = Target,
    responses(
        (status = 200, description = "Target updated successfully", body = Target),
        (status = 400, description = "Invalid target configuration"),
        (status = 404, description = "Target not found")
    ),
    security(
        ("authorization" = [])
    )
)]
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

/// Delete target
///
/// Permanently deletes a notification target.
#[utoipa::path(
    delete,
    path = "/api/v1/targets/{target_id}",
    tag = "targets",
    summary = "Delete target",
    description = "Removes a notification target from the system. Alerts using this target will need to be updated.",
    params(
        ("target_id" = String, Path, description = "Target ID (ULID format)")
    ),
    responses(
        (status = 200, description = "Target deleted successfully", body = Target),
        (status = 404, description = "Target not found")
    ),
    security(
        ("authorization" = [])
    )
)]
pub async fn delete(
    _req: HttpRequest,
    target_id: Path<Ulid>,
) -> Result<impl Responder, AlertError> {
    let target_id = target_id.into_inner();

    let target = TARGETS.delete(&target_id).await?;

    // Ok(web::Json(target.mask()))
    Ok(web::Json(target))
}
