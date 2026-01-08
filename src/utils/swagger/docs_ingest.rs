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
macro_rules! ingest_events {
    ($($item:tt)*) => {
        #[utoipa::path(
            post,
            path = "/api/v1/ingest",
            tag = "ingest",
            operation_id = "ingest",
            summary = "Ingest events",
            description = "Ingests JSON events into a log stream specified via the X-P-Stream header. Automatically creates the stream if it doesn't exist. Supports custom log sources, telemetry types, and inline log extraction. Cannot be used with internal streams or OTEL log formats (use dedicated OTEL endpoints instead).",
            params(
                ("X-P-Stream" = String, Header, description = "Name of the log stream to ingest into (required)"),
                ("X-P-Log-Source" = Option<String>, Header, description = "Log source identifier (optional, e.g., 'json', 'syslog')"),
                ("X-P-Telemetry-Type" = Option<String>, Header, description = "Telemetry type (optional, e.g., 'logs', 'metrics', 'traces')"),
                ("X-P-Meta-*" = Option<String>, Header, description = "Custom metadata fields with prefix X-P-Meta- (optional)")
            ),
            request_body(content = String, description = "JSON array of events to ingest", content_type = "application/json"),
            responses(
                (status = 200, description = "Events ingested successfully"),
                (status = 400, description = "Missing X-P-Stream header, internal stream, or OTEL format not supported"),
                (status = 500, description = "Ingestion failed")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! ingest_to_stream {
    ($($item:tt)*) => {
        #[utoipa::path(
            post,
            path = "/api/v1/logstream/{logstream}",
            tag = "ingest",
            operation_id = "post_event",
            summary = "Ingest to stream",
            description = "Ingests JSON events into the specified log stream. The stream must already exist or the request will fail. Cannot be used with internal streams. Supports custom log sources and inline log extraction via headers.",
            params(
                ("logstream" = String, Path, description = "Name of the log stream"),
                ("X-P-Log-Source" = Option<String>, Header, description = "Log source identifier (optional, e.g., 'json', 'syslog'). Defaults to 'json' if not provided"),
                ("X-P-Extract-Log" = Option<String>, Header, description = "Field path for inline log extraction (optional)"),
                ("X-P-Meta-*" = Option<String>, Header, description = "Custom metadata fields with prefix X-P-Meta- (optional)")
            ),
            request_body(content = String, content_type = "application/json"),
            responses(
                (status = 200, description = "Events ingested successfully"),
                (status = 400, description = "Internal stream or invalid data"),
                (status = 404, description = "Stream not found"),
                (status = 500, description = "Ingestion failed")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! ingest_otel_logs {
    ($($item:tt)*) => {
        #[utoipa::path(
            post,
            path = "/v1/logs",
            tag = "ingest",
            operation_id = "handle_otel_logs_ingestion",
            summary = "Ingest OTEL logs",
            description = "Ingests OpenTelemetry formatted log events. Stream name must be provided via X-P-Stream header, and log source must be set to 'otel-logs'. Creates stream if it doesn't exist. Validates that the stream is compatible with OTEL logs format.",
            params(
                ("X-P-Stream" = String, Header, description = "Name of the log stream to ingest into (required)"),
                ("X-P-Log-Source" = String, Header, description = "Must be set to 'otel-logs' for OTEL log ingestion (required)")
            ),
            request_body(content = Vec<u8>, content_type = "application/octet-stream"),
            responses(
                (status = 200, description = "OTEL logs ingested successfully"),
                (status = 400, description = "Missing headers, incorrect log source, or incompatible stream"),
                (status = 500, description = "Ingestion failed")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! ingest_otel_metrics {
    ($($item:tt)*) => {
        #[utoipa::path(
            post,
            path = "/v1/metrics",
            tag = "ingest",
            operation_id = "handle_otel_metrics_ingestion",
            summary = "Ingest OTEL metrics",
            description = "Ingests OpenTelemetry formatted metric events. Stream name must be provided via X-P-Stream header, and log source must be set to 'otel-metrics'. Creates stream if it doesn't exist. Validates that the stream is exclusively used for OTEL metrics.",
            params(
                ("X-P-Stream" = String, Header, description = "Name of the log stream to ingest into (required)"),
                ("X-P-Log-Source" = String, Header, description = "Must be set to 'otel-metrics' for OTEL metrics ingestion (required)")
            ),
            request_body(content = Vec<u8>, content_type = "application/octet-stream"),
            responses(
                (status = 200, description = "OTEL metrics ingested successfully"),
                (status = 400, description = "Missing headers, incorrect log source, or incompatible stream"),
                (status = 500, description = "Ingestion failed")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! ingest_otel_traces {
    ($($item:tt)*) => {
        #[utoipa::path(
            post,
            path = "/v1/traces",
            tag = "ingest",
            operation_id = "handle_otel_traces_ingestion",
            summary = "Ingest OTEL traces",
            description = "Ingests OpenTelemetry formatted trace events. Stream name must be provided via X-P-Stream header, and log source must be set to 'otel-traces'. Creates stream if it doesn't exist. Validates that the stream is exclusively used for OTEL traces.",
            params(
                ("X-P-Stream" = String, Header, description = "Name of the log stream to ingest into (required)"),
                ("X-P-Log-Source" = String, Header, description = "Must be set to 'otel-traces' for OTEL traces ingestion (required)")
            ),
            request_body(content = Vec<u8>, content_type = "application/octet-stream"),
            responses(
                (status = 200, description = "OTEL traces ingested successfully"),
                (status = 400, description = "Missing headers, incorrect log source, or incompatible stream"),
                (status = 500, description = "Ingestion failed")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}
