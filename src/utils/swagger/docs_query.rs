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
macro_rules! doc_execute_query {
    ($($item:tt)*) => {
        #[utoipa::path(
            post,
            path = "/api/v1/query",
            tag = "query",
            operation_id = "query",
            summary = "Execute SQL query",
            description = "Executes a SQL query against one or more log streams within the specified time range. Supports both batch and streaming modes. Returns records with optional field metadata. Validates user permissions for all accessed streams. Optimizes COUNT(*) queries for better performance.",
            request_body = serde_json::Value,
            responses(
                (status = 200, description = "Query executed successfully", body = serde_json::Value),
                (status = 400, description = "Invalid query or time range"),
                (status = 403, description = "Insufficient permissions for queried streams"),
                (status = 404, description = "Stream not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_get_counts {
    ($($item:tt)*) => {
        #[utoipa::path(
            post,
            path = "/api/v1/counts",
            tag = "query",
            operation_id = "get_counts",
            summary = "Get event counts",
            description = "Retrieves event counts for a stream within a time range, divided into bins. Optionally applies SQL filters and grouping. Useful for generating histograms and time-series visualizations. Validates user permissions for the queried stream.",
            request_body = serde_json::Value,
            responses(
                (status = 200, description = "Event counts retrieved successfully", body = serde_json::Value),
                (status = 400, description = "Invalid request or time range"),
                (status = 403, description = "Insufficient permissions"),
                (status = 404, description = "Stream not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}
