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
macro_rules! doc_list_logstreams {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/logstream",
            tag = "logstream",
            operation_id = "list_logstreams",
            summary = "List all log streamssss",
            description = "Returns an array of log streams accessible to the current user based on their role permissions.",
            responses(
                (status = 200, description = "List of accessible streams", body = Vec<StreamListEntry>),
                (status = 401, description = "Unauthorized")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_delete_logstream {
    ($($item:tt)*) => {
        #[utoipa::path(
            delete,
            path = "/api/v1/logstream/{logstream}",
            tag = "logstream",
            operation_id = "delete_logstream",
            summary = "Delete log stream",
            description = "Permanently deletes a log stream and all associated data from storage, staging, and hot tier. This operation cannot be undone.",
            params(
                ("logstream" = String, Path, description = "Name of the log stream to delete")
            ),
            responses(
                (status = 200, description = "Stream deleted successfully", body = String),
                (status = 404, description = "Stream not found"),
                (status = 500, description = "Internal server error")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_detect_logstream_schema {
    ($($item:tt)*) => {
        #[utoipa::path(
            post,
            path = "/api/v1/logstream/schema/detect",
            tag = "logstream",
            operation_id = "detect_schema",
            summary = "Detect schema from logs",
            description = "Analyzes sample JSON log data and returns a suggested Arrow schema. Useful for creating new log streams with appropriate field types. Automatically flattens nested JSON up to the configured depth limit.",
            request_body = serde_json::Value,
            responses(
                (status = 200, description = "Detected schema", body = serde_json::Value),
                (status = 400, description = "Invalid JSON data, schema detection failed, or JSON too deeply nested")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_get_logstream_schema {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/logstream/{logstream}/schema",
            tag = "logstream",
            operation_id = "get_logstream_schema",
            summary = "Get stream schema",
            description = "Returns the Arrow schema definition including field names, types, and metadata for the log stream. In distributed mode, ensures schema is synchronized from ingestors.",
            params(
                ("logstream" = String, Path, description = "Name of the log stream")
            ),
            responses(
                (status = 200, description = "Stream schema", body = serde_json::Value),
                (status = 404, description = "Stream not found"),
                (status = 417, description = "Failed to update schema in distributed mode")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_put_logstream {
    ($($item:tt)*) => {
        #[utoipa::path(
            put,
            path = "/api/v1/logstream/{logstream}",
            tag = "logstream",
            operation_id = "put_stream",
            summary = "Create or update stream",
            description = "Creates a new log stream with the specified schema or updates an existing stream's configuration. Schema and settings are provided in the request body as JSON. Supports custom time partitioning and retention policies.",
            params(
                ("logstream" = String, Path, description = "Name of the log stream to create or update"),
                ("X-P-Time-Partition" = Option<String>, Header, description = "Time partition field name (optional, e.g., 'timestamp'). Defines which field to use for time-based partitioning"),
                ("X-P-Time-Partition-Limit" = Option<String>, Header, description = "Time partition limit (optional, e.g., '1d', '1h'). Sets the granularity of time partitions"),
                ("X-P-Custom-Partition" = Option<String>, Header, description = "Custom partition field (optional). Additional field to partition data by"),
                ("X-P-Static-Schema-Flag" = Option<String>, Header, description = "Enable static schema mode (optional, set to 'true'). When enabled, schema changes are not allowed"),
                ("X-P-Update-Stream" = Option<String>, Header, description = "Update existing stream (optional, set to 'true'). Allows modifying an existing stream's configuration"),
                ("X-P-Stream-Type" = Option<String>, Header, description = "Stream type (optional, e.g., 'user-defined', 'internal'). Defaults to 'user-defined'"),
                ("X-P-Log-Source" = Option<String>, Header, description = "Log source identifier (optional, e.g., 'json', 'syslog', 'otel-logs'). Defaults to 'json'"),
                ("X-P-Telemetry-Type" = Option<String>, Header, description = "Telemetry type (optional, e.g., 'logs', 'metrics', 'traces'). Defaults to 'logs'")
            ),
            request_body(content = String, content_type = "application/json"),
            responses(
                (status = 200, description = "Stream created or updated successfully"),
                (status = 400, description = "Invalid schema or configuration"),
                (status = 409, description = "Stream already exists (on create)")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_get_logstream_retention {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/logstream/{logstream}/retention",
            tag = "logstream",
            operation_id = "get_logstream_retention",
            summary = "Get retention policy",
            description = "Returns the retention duration configured for the log stream (e.g., '30d', '7d'). Returns empty if no custom retention is set.",
            params(
                ("logstream" = String, Path, description = "Name of the log stream")
            ),
            responses(
                (status = 200, description = "Retention policy", body = Retention),
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
macro_rules! doc_put_logstream_retention {
    ($($item:tt)*) => {
        #[utoipa::path(
            put,
            path = "/api/v1/logstream/{logstream}/retention",
            tag = "logstream",
            operation_id = "put_retention",
            summary = "Set retention policy",
            description = "Sets the data retention policy for a log stream. Data older than the retention period will be automatically deleted. Accepts duration in various formats (e.g., '30d', '7d', '24h').",
            params(
                ("logstream" = String, Path, description = "Name of the log stream")
            ),
            request_body = $crate::storage::retention::Retention,
            responses(
                (status = 200, description = "Retention policy updated successfully", body = String),
                (status = 400, description = "Invalid retention format"),
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
macro_rules! doc_get_logstream_stats {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/logstream/{logstream}/stats",
            tag = "logstream",
            operation_id = "get_logstream_stats",
            summary = "Get stream statistics",
            description = "Returns detailed statistics including event counts, ingestion size, storage size, and query metrics. Optionally filter by date using 'date' query parameter (format: YYYY-MM-DD). In distributed mode, aggregates stats from all ingestor nodes.",
            params(
                ("logstream" = String, Path, description = "Name of the log stream"),
                ("date" = Option<String>, Query, description = "Optional date filter for historical stats (YYYY-MM-DD)")
            ),
            responses(
                (status = 200, description = "Stream statistics", body = QueriedStats),
                (status = 404, description = "Stream not found"),
                (status = 400, description = "Invalid date format")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_get_logstream_info {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/logstream/{logstream}/info",
            tag = "logstream",
            operation_id = "get_logstream_info",
            summary = "Get stream information",
            description = "Retrieves comprehensive stream metadata including creation timestamp, first/latest event times, time partition settings, custom partition configuration, static schema flag, log source, and telemetry type.",
            params(
                ("logstream" = String, Path, description = "Name of the log stream")
            ),
            responses(
                (status = 200, description = "Stream information", body = StreamInfo),
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
macro_rules! doc_put_logstream_hottier {
    ($($item:tt)*) => {
        #[utoipa::path(
            put,
            path = "/api/v1/logstream/{logstream}/hottier",
            tag = "logstream",
            operation_id = "put_stream_hot_tier",
            summary = "Configure stream hot tier",
            description = "Configures hot tier settings for the stream. Hot tier keeps recent data in faster storage for improved query performance. Validates size limits and checks for conflicts with internal streams. Cannot be used with internal streams.",
            params(
                ("logstream" = String, Path, description = "Name of the log stream")
            ),
            request_body = StreamHotTier,
            responses(
                (status = 200, description = "Hot tier configured successfully", body = String),
                (status = 400, description = "Invalid hot tier configuration or internal stream"),
                (status = 403, description = "Hot tier not enabled at server level"),
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
macro_rules! doc_get_logstream_hottier {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/logstream/{logstream}/hottier",
            tag = "logstream",
            operation_id = "get_stream_hot_tier",
            summary = "Get stream hot tier configuration",
            description = "Returns hot tier metadata including size, used size, available size, version, and oldest data timestamp.",
            params(
                ("logstream" = String, Path, description = "Name of the log stream")
            ),
            responses(
                (status = 200, description = "Hot tier configuration", body = StreamHotTier),
                (status = 403, description = "Hot tier not enabled at server level"),
                (status = 404, description = "Stream not found or hot tier not configured")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_delete_logstream_hottier {
    ($($item:tt)*) => {
        #[utoipa::path(
            delete,
            path = "/api/v1/logstream/{logstream}/hottier",
            tag = "logstream",
            operation_id = "delete_stream_hot_tier",
            summary = "Delete stream hot tier",
            description = "Removes hot tier configuration and data for the stream. Data will still be available in cold storage. Cannot be used with internal streams.",
            params(
                ("logstream" = String, Path, description = "Name of the log stream")
            ),
            responses(
                (status = 200, description = "Hot tier deleted successfully", body = String),
                (status = 400, description = "Cannot delete hot tier for internal stream"),
                (status = 403, description = "Hot tier not enabled at server level"),
                (status = 404, description = "Stream not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}
