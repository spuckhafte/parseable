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
macro_rules! doc_list_correlations {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/correlations",
            tag = "correlations",
            operation_id = "list_correlations",
            summary = "List correlations",
            description = "Retrieves all correlation configurations created by or shared with the current user.",
            responses(
                (status = 200, description = "List of correlations", body = Vec<CorrelationConfig>)
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_get_correlation_by_id {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/correlations/{correlation_id}",
            tag = "correlations",
            operation_id = "get_correlation",
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
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_create_correlation {
    ($($item:tt)*) => {
        #[utoipa::path(
            post,
            path = "/api/v1/correlations",
            tag = "correlations",
            operation_id = "create_correlation",
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
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_modify_correlation {
    ($($item:tt)*) => {
        #[utoipa::path(
            put,
            path = "/api/v1/correlations/{correlation_id}",
            tag = "correlations",
            operation_id = "modify_correlation",
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
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_delete_correlation {
    ($($item:tt)*) => {
        #[utoipa::path(
            delete,
            path = "/api/v1/correlations/{correlation_id}",
            tag = "correlations",
            operation_id = "delete_correlation",
            summary = "Delete correlation",
            description = "Removes a correlation configuration from the system. This operation cannot be undone.",
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
        $($item)*
    };
}
