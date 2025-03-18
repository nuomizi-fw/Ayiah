use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, decode, encode};
use serde::{Deserialize, Serialize};

use crate::{
    app::config::ConfigManager,
    error::{AuthError, AyiahError},
};

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthBody {
    pub token: String,
    pub token_type: String,
}

impl AuthBody {
    pub fn new(token: String) -> Self {
        Self {
            token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    // Subject
    pub sub: String,
    // Expiration Time
    pub exp: i64,
    // Issuer
    pub iss: String,
    // Not Before
    pub nbf: i64,
}

impl JwtClaims {
    pub fn new(sub: String) -> Self {
        let now = chrono::Utc::now();
        let config = ConfigManager::instance()
            .expect("Configuration not initialized")
            .read();

        JwtClaims {
            sub,
            exp: (now + chrono::Duration::hours(config.auth.jwt_expiry_hours as i64)).timestamp(),
            iss: "ComikNet".to_string(),
            nbf: now.timestamp(),
        }
    }

    pub fn encode_jwt(&self) -> Result<String, AuthError> {
        let config = ConfigManager::instance()
            .expect("Configuration not initialized")
            .read();

        encode(
            &Header::new(Algorithm::default()),
            &self,
            &EncodingKey::from_secret(config.auth.jwt_secret.as_bytes()),
        )
        .map_err(|_| AuthError::TokenCreation)
    }

    fn decode_jwt(token: &str) -> Result<Self, AuthError> {
        let config = ConfigManager::instance()
            .expect("Configuration not initialized")
            .read();

        decode::<Self>(
            token,
            &DecodingKey::from_secret(config.auth.jwt_secret.as_bytes()),
            &jsonwebtoken::Validation::new(Algorithm::default()),
        )
        .map(|data| data.claims)
        .map_err(|_| AuthError::InvalidToken)
    }
}

impl<S> FromRequestParts<S> for JwtClaims
where
    S: Send + Sync,
{
    type Rejection = AyiahError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AyiahError::AuthError(AuthError::InvalidToken))?;
        // Decode the user data

        let token_data = JwtClaims::decode_jwt(bearer.token())
            .map_err(|_| AyiahError::AuthError(AuthError::InvalidToken))?;

        Ok(token_data)
    }
}
