use std::collections::HashMap;

use nebula_token::claim::Role;
use serde::{Deserialize, Serialize};

pub mod saml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Identity {
    pub user_id: String,
    pub role: Role,
    pub claims: HashMap<String, String>,
}

impl Identity {
    pub fn new(user_id: String, role: Role, claims: HashMap<String, String>) -> Self {
        Self { user_id, role, claims }
    }
}
