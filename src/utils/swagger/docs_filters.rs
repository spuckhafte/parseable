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
macro_rules! doc_list_filters {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/filters",
            tag = "filters",
            operation_id = "list_filters",
            summary = "List filters",
            description = "Retrieves all filters created by or shared with the current user.",
            responses(
                (status = 200, description = "List of filters", body = Vec<Filter>)
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_get_filter_by_id {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/filters/{filter_id}",
            tag = "filters",
            operation_id = "get_filter",
            summary = "Get filter",
            description = "Returns complete filter configuration including query and stream information. Requires ownership or admin access.",
            params(
                ("filter_id" = String, Path, description = "Filter ID")
            ),
            responses(
                (status = 200, description = "Filter details", body = Filter),
                (status = 403, description = "Insufficient permissions"),
                (status = 404, description = "Filter not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_create_filter {
    ($($item:tt)*) => {
        #[utoipa::path(
            post,
            path = "/api/v1/filters",
            tag = "filters",
            operation_id = "create_filter",
            summary = "Create filter",
            description = "Creates a new filter for the current user. Validates user has permissions for the specified stream and query.",
            request_body = Filter,
            responses(
                (status = 200, description = "Filter created successfully", body = Filter),
                (status = 400, description = "Invalid filter data"),
                (status = 403, description = "Insufficient permissions for stream")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_update_filter {
    ($($item:tt)*) => {
        #[utoipa::path(
            put,
            path = "/api/v1/filters/{filter_id}",
            tag = "filters",
            operation_id = "update_filter",
            summary = "Update filter",
            description = "Modifies filter query, stream, or other properties. Requires ownership or admin access.",
            params(
                ("filter_id" = String, Path, description = "Filter ID")
            ),
            request_body = Filter,
            responses(
                (status = 200, description = "Filter updated successfully", body = Filter),
                (status = 400, description = "Invalid filter data"),
                (status = 403, description = "Insufficient permissions"),
                (status = 404, description = "Filter not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_delete_filter {
    ($($item:tt)*) => {
        #[utoipa::path(
            delete,
            path = "/api/v1/filters/{filter_id}",
            tag = "filters",
            operation_id = "delete_filter",
            summary = "Delete filter",
            description = "Removes a filter from storage. Requires ownership or admin permissions.",
            params(
                ("filter_id" = String, Path, description = "Filter ID")
            ),
            responses(
                (status = 200, description = "Filter deleted successfully"),
                (status = 403, description = "Insufficient permissions"),
                (status = 404, description = "Filter not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}
