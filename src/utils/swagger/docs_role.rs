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
macro_rules! list_roles {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/role",
            tag = "role",
            operation_id = "list_roles",
            summary = "List role names",
            description = "Retrieves an array of all role names configured in the system.",
            responses(
                (status = 200, description = "List of role names", body = Vec<String>)
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! get_role_by_name {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/role/{name}",
            tag = "role",
            operation_id = "get_role",
            summary = "Get role details",
            description = "Returns the list of privileges assigned to the specified role. If the role doesn't exist, returns an empty array.",
            params(
                ("name" = String, Path, description = "Role name")
            ),
            responses(
                (status = 200, description = "Role privileges", body = Vec<DefaultPrivilege>)
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! put_role {
    ($($item:tt)*) => {
        #[utoipa::path(
            put,
            path = "/api/v1/role/{name}",
            tag = "role",
            operation_id = "put_role",
            summary = "Create or update role",
            description = "Creates a new role with the specified privileges or updates an existing role. Automatically refreshes sessions for all users assigned this role. Role privileges define stream access permissions and allowed operations.",
            params(
                ("name" = String, Path, description = "Role name")
            ),
            request_body = Vec<DefaultPrivilege>,
            responses(
                (status = 200, description = "Role created or updated successfully"),
                (status = 400, description = "Invalid role name")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! delete_role {
    ($($item:tt)*) => {
        #[utoipa::path(
            delete,
            path = "/api/v1/role/{name}",
            tag = "role",
            operation_id = "delete_role",
            summary = "Delete role",
            description = "Removes a role from the system. The role must not be assigned to any users or user groups. This operation cannot be undone.",
            params(
                ("name" = String, Path, description = "Role name to delete")
            ),
            responses(
                (status = 200, description = "Role deleted successfully"),
                (status = 400, description = "Role is in use by users or groups")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! list_roles_detailed {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/roles",
            tag = "role",
            operation_id = "list_roles_detailed",
            summary = "List roles with details",
            description = "Retrieves a map of all roles with their associated privilege configurations. Provides more detail than /api/v1/role.",
            responses(
                (status = 200, description = "All roles with privileges", body = serde_json::Value)
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! get_default_role {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/role/default",
            tag = "role",
            operation_id = "get_default_role",
            summary = "Get default role",
            description = "Retrieves the role name that is automatically assigned to new users, or null if no default is set.",
            responses(
                (status = 200, description = "Default role name or null", body = serde_json::Value)
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! put_default_role {
    ($($item:tt)*) => {
        #[utoipa::path(
            put,
            path = "/api/v1/role/default",
            tag = "role",
            operation_id = "put_default_role",
            summary = "Set default role",
            description = "Sets the role that will be automatically assigned to newly created users. The role must exist in the system.",
            request_body(content = String, content_type = "text/plain"),
            responses(
                (status = 200, description = "Default role set successfully")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

// ============================================================================
// ALERTS ENDPOINTS
// ============================================================================
