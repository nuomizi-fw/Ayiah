use axum::Router;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

#[derive(OpenApi)]
#[openapi(paths(), components(schemas()), tags(
    (name = "auth", description = "Authentication APIs"),
))]
struct ApiDoc;

pub fn mount() -> Router {
    Router::new().merge(Scalar::with_url("/openapi", ApiDoc::openapi()))
}
