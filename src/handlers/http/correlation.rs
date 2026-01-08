/*
 * Parseable Server (C) 2022 - 2024 Parseable, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use actix_web::web::{Json, Path};
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use anyhow::Error;
use itertools::Itertools;

use crate::rbac::Users;
use crate::utils::actix::extract_session_key_from_req;
use crate::utils::{get_hash, get_user_from_request, user_auth_for_datasets};

use crate::correlation::{CORRELATIONS, CorrelationConfig, CorrelationError};

/// List correlations
///
/// Returns all correlations accessible by the current user.
#[utoipa::path(
    get,
    path = "/api/v1/correlations",
    tag = "correlations",
    summary = "List correlations",
    description = "Retrieves all correlation configurations created by or shared with the current user.",
    responses(
        (status = 200, description = "List of correlations", body = Vec<CorrelationConfig>)
    ),
    security(
        ("authorization" = [])
    )
)]
pub async fn list(req: HttpRequest) -> Result<impl Responder, CorrelationError> {
    let session_key = extract_session_key_from_req(&req)
        .map_err(|err| CorrelationError::AnyhowError(Error::msg(err.to_string())))?;

    let correlations = CORRELATIONS.list_correlations(&session_key).await?;

    Ok(web::Json(correlations))
}

/// Get correlation
///
/// Retrieves a specific correlation by ID.
#[utoipa::path(
    get,
    path = "/api/v1/correlations/{correlation_id}",
    tag = "correlations",
    summary = "Get correlation",
    description = "Returns complete correlation configuration including table joins and filters. Validates user has permissions for all referenced streams.",
    params(
        ("correlation_id" = String, Path, description = "Correlation ID")
    ),
    responses(
        (status = 200, description = "Correlation details", body = CorrelationConfig),
        (status = 403, description = "Insufficient permissions for referenced streams"),
        (status = 404, description = "Correlation not found")
    ),
    security(
        ("authorization" = [])
    )
)]
pub async fn get(
    req: HttpRequest,
    correlation_id: Path<String>,
) -> Result<impl Responder, CorrelationError> {
    let correlation_id = correlation_id.into_inner();
    let session_key = extract_session_key_from_req(&req)
        .map_err(|err| CorrelationError::AnyhowError(Error::msg(err.to_string())))?;

    let correlation = CORRELATIONS.get_correlation(&correlation_id).await?;

    let permissions = Users.get_permissions(&session_key);

    let tables = &correlation
        .table_configs
        .iter()
        .map(|t| t.table_name.clone())
        .collect_vec();

    user_auth_for_datasets(&permissions, tables).await?;

    Ok(web::Json(correlation))
}

/// Create correlation
///
/// Creates a new correlation between multiple streams.
#[utoipa::path(
    post,
    path = "/api/v1/correlations",
    tag = "correlations",
    summary = "Create correlation",
    description = "Creates a new correlation configuration that joins multiple log streams. Validates user has permissions for all referenced streams.",
    request_body = CorrelationConfig,
    responses(
        (status = 200, description = "Correlation created successfully", body = CorrelationConfig),
        (status = 400, description = "Invalid correlation configuration"),
        (status = 403, description = "Insufficient permissions for referenced streams")
    ),
    security(
        ("authorization" = [])
    )
)]
pub async fn post(
    req: HttpRequest,
    Json(mut correlation): Json<CorrelationConfig>,
) -> Result<impl Responder, CorrelationError> {
    let session_key = extract_session_key_from_req(&req)
        .map_err(|err| CorrelationError::AnyhowError(anyhow::Error::msg(err.to_string())))?;
    let user_id = get_user_from_request(&req)
        .map(|s| get_hash(&s.to_string()))
        .map_err(|err| CorrelationError::AnyhowError(Error::msg(err.to_string())))?;
    correlation.user_id = user_id;

    let correlation = CORRELATIONS.create(correlation, &session_key).await?;

    Ok(web::Json(correlation))
}

/// Modify correlation
///
/// Updates an existing correlation's configuration.
#[utoipa::path(
    put,
    path = "/api/v1/correlations/{correlation_id}",
    tag = "correlations",
    summary = "Modify correlation",
    description = "Updates correlation configuration including table joins and filters. Validates user has permissions for all referenced streams.",
    params(
        ("correlation_id" = String, Path, description = "Correlation ID")
    ),
    request_body = CorrelationConfig,
    responses(
        (status = 200, description = "Correlation updated successfully", body = CorrelationConfig),
        (status = 400, description = "Invalid correlation configuration"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Correlation not found")
    ),
    security(
        ("authorization" = [])
    )
)]
pub async fn modify(
    req: HttpRequest,
    correlation_id: Path<String>,
    Json(mut correlation): Json<CorrelationConfig>,
) -> Result<impl Responder, CorrelationError> {
    correlation.id = correlation_id.into_inner();
    correlation.user_id = get_user_from_request(&req)
        .map(|s| get_hash(&s.to_string()))
        .map_err(|err| CorrelationError::AnyhowError(Error::msg(err.to_string())))?;

    let session_key = extract_session_key_from_req(&req)
        .map_err(|err| CorrelationError::AnyhowError(anyhow::Error::msg(err.to_string())))?;

    let correlation = CORRELATIONS.update(correlation, &session_key).await?;

    Ok(web::Json(correlation))
}

/// Delete correlation
///
/// Permanently deletes a correlation.
#[utoipa::path(
    delete,
    path = "/api/v1/correlations/{correlation_id}",
    tag = "correlations",
    summary = "Delete correlation",
    description = "Removes a correlation configuration from the system.",
    params(
        ("correlation_id" = String, Path, description = "Correlation ID")
    ),
    responses(
        (status = 200, description = "Correlation deleted successfully"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Correlation not found")
    ),
    security(
        ("authorization" = [])
    )
)]
pub async fn delete(
    req: HttpRequest,
    correlation_id: Path<String>,
) -> Result<impl Responder, CorrelationError> {
    let correlation_id = correlation_id.into_inner();
    let user_id = get_user_from_request(&req)
        .map(|s| get_hash(&s.to_string()))
        .map_err(|err| CorrelationError::AnyhowError(Error::msg(err.to_string())))?;

    CORRELATIONS.delete(&correlation_id, &user_id).await?;

    Ok(HttpResponse::Ok().finish())
}
