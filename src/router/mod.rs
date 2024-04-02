mod server;
mod todos;

use crate::{handler::openapi::ApiDoc, server::AppState};
use axum::Router;
use http::header;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, decompression::DecompressionLayer,
    propagate_header::PropagateHeaderLayer, trace,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn init(state: AppState) -> Router {
    let router = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
    let router = server::add_routers(router);

    let api_router = Router::new();
    let api_router = todos::add_routers(api_router);
    let router = router.nest("/api", api_router);

    router
        .with_state(state)
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().include_headers(false))
                .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO))
                .on_failure(trace::DefaultOnFailure::new().level(tracing::Level::WARN)),
        )
        .layer(DecompressionLayer::new())
        .layer(CompressionLayer::new())
        .layer(PropagateHeaderLayer::new(header::HeaderName::from_static(
            "x-request-id",
        )))
        // TODO be more restrictive
        .layer(CorsLayer::permissive())
}
