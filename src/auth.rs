use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::{header::AUTHORIZATION, StatusCode},
};
use sqlx::{query_as, PgPool};

pub struct AuthorizedUser {
    _who: String,
}

#[async_trait]
impl<B> FromRequest<B> for AuthorizedUser
where
    B: Send, // required by `async_trait`
{
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let db = req.extensions().get::<PgPool>().unwrap();
        let auth_header: &str = req
            .headers()
            .get(AUTHORIZATION)
            .ok_or(StatusCode::UNAUTHORIZED)?
            .to_str()
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let decoded = base64::decode(auth_header)
            .map_err(|_| StatusCode::UNAUTHORIZED)?
            .split(|&c| c == ':' as u8)
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
        .fetch_one(db)
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
