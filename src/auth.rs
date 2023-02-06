use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
};
use base64::{prelude::BASE64_STANDARD, Engine};
use sqlx::{query_as, PgPool};

pub struct AuthorizedUser {
    _who: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthorizedUser
where
    PgPool: FromRef<S>,
    S: Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header: &str = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(StatusCode::UNAUTHORIZED)?
            .to_str()
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let decoded = BASE64_STANDARD
            .decode(auth_header)
            .map_err(|_| StatusCode::UNAUTHORIZED)?
            .split(|&c| c == b':')
            .map(|b| String::from_utf8(b.into()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let (username, key) = match &*decoded {
            [username, key] => (username, key),
            _ => return Err(StatusCode::UNAUTHORIZED),
        };

        struct Exists {
            exists: bool,
        }

        match query_as!(
            Exists,
            r#"
            SELECT
                EXISTS(
                    SELECT 1
                    FROM public.authentication_keys
                    WHERE
                        username = $1
                    AND
                        key = $2
                )
                AS "exists!"
            "#,
            username,
            key
        )
        .fetch_one(&PgPool::from_ref(state))
        .await
        {
            Ok(Exists { exists: true }) => Ok(AuthorizedUser {
                _who: username.clone(),
            }),
            Ok(Exists { exists: false }) => Err(StatusCode::UNAUTHORIZED),
            Err(why) => {
                tracing::error!("error selecting from the database: {}", why);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}
