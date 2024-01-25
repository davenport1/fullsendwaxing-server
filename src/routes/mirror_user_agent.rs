use axum_extra::TypedHeader;
use axum_extra::headers;

pub async fn mirror_user_agent(TypedHeader(user_agent): TypedHeader<headers::UserAgent>) -> String {
    user_agent.to_string()
}