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
macro_rules! doc_create_target {
    ($($item:tt)*) => {
        #[utoipa::path(
            post,
            path = "/api/v1/targets",
            tag = "targets",
            operation_id = "create_target",
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
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_list_targets {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/targets",
            tag = "targets",
            operation_id = "list_targets",
            summary = "List targets",
            description = "Retrieves all notification targets configured in the system.",
            responses(
                (status = 200, description = "List of targets", body = Vec<Target>)
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_get_target_by_id {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/targets/{target_id}",
            tag = "targets",
            operation_id = "get_target",
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
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_update_target {
    ($($item:tt)*) => {
        #[utoipa::path(
            put,
            path = "/api/v1/targets/{target_id}",
            tag = "targets",
            operation_id = "update_target",
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
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_delete_target {
    ($($item:tt)*) => {
        #[utoipa::path(
            delete,
            path = "/api/v1/targets/{target_id}",
            tag = "targets",
            operation_id = "delete_target",
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
        $($item)*
    };
}
