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

// Documentation macro modules - these modules contain macro_export macros
// that are automatically available at the crate root

#[macro_use]
pub mod docs_logstream;
#[macro_use]
pub mod docs_role;
#[macro_use]
pub mod docs_alerts;
#[macro_use]
pub mod docs_filters;
#[macro_use]
pub mod docs_dashboards;
#[macro_use]
pub mod docs_rbac;
#[macro_use]
pub mod docs_query;
#[macro_use]
pub mod docs_ingest;

use utoipa;
use utoipa::Modify;
use utoipa::OpenApi;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "authorization",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Basic)
                        .description(Some(
                            "Default credentials: username=[admin], password=[admin]",
                        ))
                        .build(),
                ),
            );
        }
    }
}

/// OpenAPI documentation for Parseable API
#[derive(OpenApi)]
#[openapi(
    servers(
        (url = "http://localhost:8000", description = "Default local server"),
        (url = "{custom-domain}", description = "Custom server domain",
            variables(
                ("custom-domain" = (default = "https://parseable-instance.com", description = "Your Parseable server domain"))
            )
        )
    ),
    paths(
        // Logstream Management
        crate::handlers::http::logstream::delete,
        crate::handlers::http::logstream::list,
        crate::handlers::http::logstream::detect_schema,
        crate::handlers::http::logstream::get_schema,
        crate::handlers::http::logstream::put_stream,
        crate::handlers::http::logstream::get_retention,
        crate::handlers::http::logstream::put_retention,
        crate::handlers::http::logstream::get_stats,
        crate::handlers::http::logstream::get_stream_info,
        crate::handlers::http::logstream::put_stream_hot_tier,
        crate::handlers::http::logstream::get_stream_hot_tier,
        crate::handlers::http::logstream::delete_stream_hot_tier,
        // RBAC - User Management
        crate::handlers::http::rbac::list_users,
        crate::handlers::http::rbac::list_users_prism,
        crate::handlers::http::rbac::get_prism_user,
        crate::handlers::http::rbac::post_user,
        crate::handlers::http::rbac::post_gen_password,
        crate::handlers::http::rbac::get_role,
        crate::handlers::http::rbac::delete_user,
        crate::handlers::http::rbac::add_roles_to_user,
        crate::handlers::http::rbac::remove_roles_from_user,
        // RBAC - Role Management
        crate::handlers::http::role::put,
        crate::handlers::http::role::get,
        crate::handlers::http::role::list,
        crate::handlers::http::role::list_roles,
        crate::handlers::http::role::delete,
        crate::handlers::http::role::put_default,
        crate::handlers::http::role::get_default,
        // Ingestion
        crate::handlers::http::ingest::ingest,
        crate::handlers::http::ingest::handle_otel_logs_ingestion,
        crate::handlers::http::ingest::handle_otel_metrics_ingestion,
        crate::handlers::http::ingest::handle_otel_traces_ingestion,
        crate::handlers::http::ingest::post_event,
        // Query
        crate::handlers::http::query::query,
        crate::handlers::http::query::get_counts,
        // Alerts
        crate::handlers::http::alerts::list,
        crate::handlers::http::alerts::post,
        crate::handlers::http::alerts::get,
        crate::handlers::http::alerts::delete,
        crate::handlers::http::alerts::modify_alert,
        crate::handlers::http::alerts::disable_alert,
        crate::handlers::http::alerts::enable_alert,
        crate::handlers::http::alerts::list_tags,
        // Dashboards
        crate::handlers::http::users::dashboards::list_dashboards,
        crate::handlers::http::users::dashboards::get_dashboard,
        crate::handlers::http::users::dashboards::create_dashboard,
        crate::handlers::http::users::dashboards::update_dashboard,
        // Filters
        crate::handlers::http::users::filters::list,
        crate::handlers::http::users::filters::get,
        crate::handlers::http::users::filters::post,
        crate::handlers::http::users::filters::update,
        crate::handlers::http::users::filters::delete,
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "logstream", description = "Log stream management - create, delete, configure streams"),
        (name = "rbac", description = "Role-Based Access Control - user and permission management"),
        (name = "role", description = "Role management - create, update, delete roles"),
        (name = "ingest", description = "Log ingestion - ingest JSON and OTEL formatted logs"),
        (name = "query", description = "Query execution - run SQL queries and get event counts"),
        (name = "alerts", description = "Alert management - configure and monitor alerts"),
        (name = "dashboards", description = "Dashboard management - create and manage dashboards"),
        (name = "filters", description = "Filter management - create and manage saved filters"),
    ),
    info(
        title = "Parseable Server API",
        version = "2.5.4",
        description = "Comprehensive API documentation for interacting with Parseable's server.",
        contact(
            name = "Parseable",
            url = "https://www.parseable.com",
        )
    ),
    security(
        ("authorization" = [])
    )
)]
pub struct ApiDoc;
