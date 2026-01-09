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
macro_rules! doc_list_users {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/user",
            tag = "rbac",
            operation_id = "list_users",
            summary = "List all users",
            description = "Returns an array of all registered users showing their user ID and authentication method (native or oauth).",
            responses(
                (status = 200, description = "List of users", body = Vec<serde_json::Value>)
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_list_users_detailed {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/users",
            tag = "rbac",
            operation_id = "list_users_prism",
            summary = "List users with details",
            description = "Returns detailed information for all registered users including their roles, user groups, and permissions. This endpoint provides more comprehensive data than /api/v1/user.",
            responses(
                (status = 200, description = "Detailed user list", body = Vec<serde_json::Value>)
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_get_user_by_name {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/users/{username}",
            tag = "rbac",
            operation_id = "get_prism_user",
            summary = "Get user details",
            description = "Retrieves comprehensive information about a specific user including their roles, user groups, and privilege assignments.",
            params(
                ("username" = String, Path, description = "Username to retrieve")
            ),
            responses(
                (status = 200, description = "User details", body = serde_json::Value),
                (status = 404, description = "User not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_create_user {
    ($($item:tt)*) => {
        #[utoipa::path(
            post,
            path = "/api/v1/user/{username}",
            tag = "rbac",
            operation_id = "post_user",
            summary = "Create user",
            description = "Creates a new user account with the specified username. Optionally accepts a JSON array of role names to assign. Validates that all specified roles exist. Returns a generated password that should be shared securely with the user.",
            params(
                ("username" = String, Path, description = "Username for the new user")
            ),
            request_body = serde_json::Value,
            responses(
                (status = 200, description = "User created successfully, returns generated password", body = String),
                (status = 400, description = "User already exists, invalid username, or roles don't exist")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_reset_user_password {
    ($($item:tt)*) => {
        #[utoipa::path(
            post,
            path = "/api/v1/user/{username}/generate-new-password",
            tag = "rbac",
            operation_id = "post_gen_password",
            summary = "Reset user password",
            description = "Generates a new random password for the specified user and updates their account. Returns the new password which should be shared securely with the user. Only works for native authentication users.",
            params(
                ("username" = String, Path, description = "Username to reset password for")
            ),
            responses(
                (status = 200, description = "New password generated", body = String),
                (status = 404, description = "User not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_get_user_roles {
    ($($item:tt)*) => {
        #[utoipa::path(
            get,
            path = "/api/v1/user/{userid}/role",
            tag = "rbac",
            operation_id = "get_role",
            summary = "Get user roles",
            description = "Retrieves all roles for the specified user. Returns both direct role assignments and roles inherited from user group memberships, showing privilege details for each role.",
            params(
                ("userid" = String, Path, description = "User ID to get roles for")
            ),
            responses(
                (status = 200, description = "User roles", body = serde_json::Value),
                (status = 404, description = "User not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_add_user_roles {
    ($($item:tt)*) => {
        #[utoipa::path(
            patch,
            path = "/api/v1/user/{userid}/role/add",
            tag = "rbac",
            operation_id = "add_roles_to_user",
            summary = "Add user roles",
            description = "Assigns additional roles to a user. Validates that all specified roles exist before assignment. Roles grant privileges for accessing streams and performing operations.",
            params(
                ("userid" = String, Path, description = "User ID to add roles to")
            ),
            request_body = Vec<String>,
            responses(
                (status = 200, description = "Roles added successfully", body = String),
                (status = 400, description = "One or more roles don't exist"),
                (status = 404, description = "User not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_remove_user_roles {
    ($($item:tt)*) => {
        #[utoipa::path(
            patch,
            path = "/api/v1/user/{userid}/role/remove",
            tag = "rbac",
            operation_id = "remove_roles_from_user",
            summary = "Remove user roles",
            description = "Removes specified roles from a user. Validates that all specified roles exist and are currently assigned to the user before removal.",
            params(
                ("userid" = String, Path, description = "User ID to remove roles from")
            ),
            request_body = Vec<String>,
            responses(
                (status = 200, description = "Roles removed successfully", body = String),
                (status = 400, description = "Roles don't exist or aren't assigned to user"),
                (status = 404, description = "User not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}

#[macro_export]
macro_rules! doc_delete_user {
    ($($item:tt)*) => {
        #[utoipa::path(
            delete,
            path = "/api/v1/user/delete/{userid}",
            tag = "rbac",
            operation_id = "delete_user",
            summary = "Delete user",
            description = "Permanently removes a user account from the system. The user must first be removed from all user groups before deletion. This operation cannot be undone.",
            params(
                ("userid" = String, Path, description = "User ID to delete")
            ),
            responses(
                (status = 200, description = "User deleted successfully", body = String),
                (status = 400, description = "User is still a member of user groups"),
                (status = 404, description = "User not found")
            ),
            security(
                ("authorization" = [])
            )
        )]
        $($item)*
    };
}
