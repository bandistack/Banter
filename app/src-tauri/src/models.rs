use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OAuthTokens {
    pub access_token: String,
    pub id_token: Option<String>,
    pub scope: Option<Vec<String>>,
}

impl OAuthTokens {
    pub fn builder(access_token: String) -> OAuthTokensBuilder {
        OAuthTokensBuilder {
            access_token,
            id_token: None,
            scope: None,
        }
    }
}

pub struct OAuthTokensBuilder {
    access_token: String,
    id_token: Option<String>,
    scope: Option<Vec<String>>,
}

impl OAuthTokensBuilder {
    pub fn id_token(mut self, id_token: Option<String>) -> Self {
        self.id_token = id_token;
        self
    }

    #[allow(dead_code)]
    pub fn scope(mut self, scope: Option<Vec<String>>) -> Self {
        self.scope = scope;
        self
    }

    pub fn build(self) -> OAuthTokens {
        OAuthTokens {
            access_token: self.access_token,
            id_token: self.id_token,
            scope: self.scope,
        }
    }
}