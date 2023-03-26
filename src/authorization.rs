use crate::{validation::CustomError, AppState, Result};
use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    Extension, RequestPartsExt,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    exp: usize,
}

impl Claims {
    fn new(sub: i32) -> Self {
        Self {
            sub,
            exp: Utc::now().timestamp() as usize + Duration::weeks(2).num_seconds() as usize,
        }
    }

    pub fn to_jwt(&self, ctx: Extension<AppState>) -> Result<String> {
        let key = EncodingKey::from_secret(ctx.config.hmac_key.as_bytes());
        let token = encode(&Header::default(), self, &key).map_err(|e| {
            tracing::error!("Could not encode JWT: {}", e);
            CustomError::Anyhow(e.into())
        })?;

        Ok(token)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = CustomError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| CustomError::Unauthorized)?;

        let ctx: Extension<AppState> =
            Extension::from_request_parts(parts, state)
                .await
                .map_err(|e| {
                    tracing::error!("Could not extract app state");
                    CustomError::Anyhow(e.into())
                })?;

        let key = DecodingKey::from_secret(ctx.config.hmac_key.as_bytes());

        let token_data = decode::<Claims>(bearer.token(), &key, &Validation::default())
            .map_err(|_| CustomError::Unauthorized)?;

        Ok(token_data.claims)
    }
}
