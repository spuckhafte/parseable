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

#[macro_export]
macro_rules! list_dashboards {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/dashboards",
            tag = "dashboards",
            operation_id = "list_dashboards",
            summary = "List dashboards",
            description = "Returns all dashboards created by or shared with the current user. Supports filtering by tags and pagination via query parameters.",
            responses(
                (status = 200, description = "List of dashboards", body = Vec<Dashboard>),
                (status = 400, description = "Invalid query parameters")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! get_dashboard_by_id {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/dashboards/{dashboard_id}",
            tag = "dashboards",
            operation_id = "get_dashboard",
            summary = "Get dashboard",
            description = "Returns complete dashboard configuration including tiles and metadata. Public dashboards can be accessed by anyone, private dashboards require ownership or admin access.",
            params(
                ("dashboard_id" = String, Path, description = "Dashboard ID (ULID format)")
            ),
            responses(
                (status = 200, description = "Dashboard details", body = Dashboard),
                (status = 403, description = "Insufficient permissions"),
                (status = 404, description = "Dashboard not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! create_dashboard {
    ($($item:tt)*) => {
        #[utoipa::path(
            post,
            path = "/api/v1/dashboards",
            tag = "dashboards",
            operation_id = "create_dashboard",
            summary = "Create dashboard",
            description = "Creates a new dashboard for the current user with the provided title, tiles, and optional tags. Automatically assigns author and generates unique ID.",
            request_body = Dashboard,
            responses(
                (status = 200, description = "Dashboard created successfully", body = Dashboard),
                (status = 400, description = "Invalid dashboard data")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! update_dashboard {
    ($($item:tt)*) => {
        #[utoipa::path(
            put,
            path = "/api/v1/dashboards/{dashboard_id}",
            tag = "dashboards",
            operation_id = "update_dashboard",
            summary = "Update dashboard",
            description = "Updates dashboard properties. Can use query parameters (tags, isFavorite, renameTo) for partial updates or request body for full replacement. Cannot use both simultaneously.",
            params(
                ("dashboard_id" = String, Path, description = "Dashboard ID (ULID format)")
            ),
            request_body = Dashboard,
            responses(
                (status = 200, description = "Dashboard updated successfully", body = Dashboard),
                (status = 400, description = "Invalid update data or mixed update methods"),
                (status = 403, description = "Insufficient permissions"),
                (status = 404, description = "Dashboard not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

// ============================================================================
// RBAC ENDPOINTS
// ============================================================================
