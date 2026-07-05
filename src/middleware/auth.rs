use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{
    config::settings::Settings,
    utils::jwt::verify_token,
};

fn extract_bearer_token(header: &str) -> Option<&str> {
    let header = header.trim();
    if header.is_empty() {
        return None;
    }

    let token = header
        .strip_prefix("Bearer ")
        .or_else(|| header.strip_prefix("bearer "))
        .unwrap_or(header)
        .trim();

    (!token.is_empty()).then_some(token)
}

pub async fn auth(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let header = request
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok());

    let Some(header) = header else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let Some(token) = extract_bearer_token(header) else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let settings = Settings::new();

    let claims = verify_token(
        token,
        &settings.jwt_secret,
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_id =
        Uuid::parse_str(&claims.sub)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(user_id);

    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use super::extract_bearer_token;

    #[test]
    fn extracts_bearer_token_case_insensitively() {
        assert_eq!(extract_bearer_token("Bearer abc123"), Some("abc123"));
        assert_eq!(extract_bearer_token("bearer abc123"), Some("abc123"));
        assert_eq!(extract_bearer_token("abc123"), Some("abc123"));
    }
}