use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        description = "Open Api",
        title = "Simple File Server",
        version = "25.1.0",
    ),
    paths(
        crate::route::health_check::health_check,
        crate::route::login::login,
        crate::route::upload::upload,
    ),
    components(schemas())
)]
pub struct ApiDoc;