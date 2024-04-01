use crate::dto::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // server
        crate::handler::server::health,
        crate::handler::server::state,
    ),
    components(
        schemas(
            ServiceStatusResponse,
            ErrorResponse,
        )
    )
)]
pub struct ApiDoc;
