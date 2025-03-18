use axum::Router;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::entity::{user, user_preferences};

#[derive(OpenApi)]
#[openapi(
    info(title = "Ayiah API", version = "0.1.0", description = "Ayiah Media Server API"),
    components(
        schemas(
            // Auth schemas would go here

            // User schemas
            user::Model,

            // User preference schemas
            user_preferences::Model,
        )
    ),
    tags(

    )
)]
struct ApiDoc;

pub fn mount() -> Router {
    Router::new().merge(Scalar::with_url("/openapi", ApiDoc::openapi()))
}
