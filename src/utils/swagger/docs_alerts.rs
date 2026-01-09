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
macro_rules! doc_list_alerts {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/alerts",
            tag = "alerts",
            operation_id = "list_alerts",
            summary = "List alerts",
            description = "Returns all alerts that the user has permission to view. Supports filtering by tags, pagination, and sorting. Only returns alerts for streams the user has read access to.",
            responses(
                (status = 200, description = "List of alerts", body = Vec<serde_json::Value>),
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
macro_rules! doc_get_alert_by_id {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/alerts/{alert_id}",
            tag = "alerts",
            operation_id = "get_alert",
            summary = "Get alert",
            description = "Returns detailed configuration for the specified alert including query, thresholds, state, and notification settings. Validates user has permission for queried streams.",
            params(
                ("alert_id" = String, Path, description = "Alert ID (ULID format)")
            ),
            responses(
                (status = 200, description = "Alert details", body = serde_json::Value),
                (status = 403, description = "Insufficient permissions"),
                (status = 404, description = "Alert not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_create_alert {
    ($($item:tt)*) => {
        #[utoipa::path(
            post,
            path = "/api/v1/alerts",
            tag = "alerts",
            operation_id = "create_alert",
            summary = "Create alert",
            description = "Creates a new alert that monitors a stream for specific conditions. Validates evaluation frequency, notification intervals, user permissions for queried streams, and starts the alert monitoring task.",
            request_body = AlertRequest,
            responses(
                (status = 200, description = "Alert created successfully", body = serde_json::Value),
                (status = 400, description = "Invalid alert configuration"),
                (status = 403, description = "Insufficient permissions for queried streams")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_modify_alert {
    ($($item:tt)*) => {
        #[utoipa::path(
            put,
            path = "/api/v1/alerts/{alert_id}",
            tag = "alerts",
            operation_id = "modify_alert",
            summary = "Modify alert",
            description = "Updates alert configuration including query, thresholds, and notification settings. Stops the old task and starts a new one with updated configuration.",
            params(
                ("alert_id" = String, Path, description = "Alert ID (ULID format)")
            ),
            request_body = AlertRequest,
            responses(
                (status = 200, description = "Alert modified successfully", body = serde_json::Value),
                (status = 400, description = "Invalid configuration"),
                (status = 403, description = "Insufficient permissions"),
                (status = 404, description = "Alert not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_delete_alert {
    ($($item:tt)*) => {
        #[utoipa::path(
            delete,
            path = "/api/v1/alerts/{alert_id}",
            tag = "alerts",
            operation_id = "delete_alert",
            summary = "Delete alert",
            description = "Removes an alert from storage and memory, and stops its scheduled evaluation task. Requires user to have permissions for the streams referenced in the alert query.",
            params(
                ("alert_id" = String, Path, description = "Alert ID (ULID format)")
            ),
            responses(
                (status = 200, description = "Alert deleted successfully", body = serde_json::Value),
                (status = 403, description = "Insufficient permissions"),
                (status = 404, description = "Alert not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_disable_alert {
    ($($item:tt)*) => {
        #[utoipa::path(
            patch,
            path = "/api/v1/alerts/{alert_id}/disable",
            tag = "alerts",
            operation_id = "disable_alert",
            summary = "Disable alert",
            description = "Stops alert evaluations by setting its state to Disabled. The alert configuration remains in storage and can be re-enabled later.",
            params(
                ("alert_id" = String, Path, description = "Alert ID (ULID format)")
            ),
            responses(
                (status = 200, description = "Alert disabled successfully", body = serde_json::Value),
                (status = 403, description = "Insufficient permissions"),
                (status = 404, description = "Alert not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_enable_alert {
    ($($item:tt)*) => {
        #[utoipa::path(
            patch,
            path = "/api/v1/alerts/{alert_id}/enable",
            tag = "alerts",
            operation_id = "enable_alert",
            summary = "Enable alert",
            description = "Resumes alert evaluations by changing state from Disabled to NotTriggered. Only works on disabled alerts.",
            params(
                ("alert_id" = String, Path, description = "Alert ID (ULID format)")
            ),
            responses(
                (status = 200, description = "Alert enabled successfully", body = serde_json::Value),
                (status = 400, description = "Alert is not disabled"),
                (status = 403, description = "Insufficient permissions"),
                (status = 404, description = "Alert not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_list_alert_tags {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/alerts/list_tags",
            tag = "alerts",
            operation_id = "list_alert_tags",
            summary = "List alert tags",
            description = "Retrieves a list of all unique tags that have been assigned to alerts in the system.",
            responses(
                (status = 200, description = "List of tags", body = Vec<String>)
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}
