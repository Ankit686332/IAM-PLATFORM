use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(),
    components(),
    tags(
        (name = "IAM Backend", description = "Identity and Access Management API")
    )
)]
pub struct ApiDoc;