//! [GET /_synapse/admin/v1/users/:user_id/admin](https://github.com/matrix-org/synapse/blob/master/docs/admin_api/user_admin_api.rst#get-whether-a-user-is-a-server-administrator-or-not)

use ruma::{
    api::{metadata, request, response, Metadata},
    UserId,
};

const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/users/:user_id/admin",
    }
};

#[request]
pub struct Request<'a> {
    /// User to check.
    #[ruma_api(path)]
    pub user_id: &'a UserId,
}

#[response]
pub struct Response {
    /// Whether the requested user ID is an admin.
    pub admin: bool,
}

impl<'a> Request<'a> {
    /// Creates an `Request` with the given user ID.
    pub fn new(user_id: &'a UserId) -> Self {
        Self { user_id }
    }
}

impl Response {
    /// Creates a `Response` with the given admin flag.
    pub fn new(admin: bool) -> Self {
        Self { admin }
    }
}
