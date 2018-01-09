use diesel::pg::PgConnection;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::router::route::dispatch::{finalize_pipeline_set, new_pipeline_set};
use gotham::middleware::pipeline::new_pipeline;
use gotham_middleware_diesel::DieselMiddleware;
use gotham_middleware_tokio::TokioMiddleware;
use hyper::Method;
use tokio_core::reactor::Remote;

use super::extractors::ImportRequestPath;
use super::handlers::ImportController;
use super::middlewares::ImportServiceMiddleware;
use service::ImportService;

pub fn build_app_router(
    datbase_url: &str,
    store_uri: &str,
    library_uri: &str,
    handle: Remote,
) -> Router {
    trace!("build pipelines");
    let pipelines = new_pipeline_set();
    let (pipelines, default) = pipelines.add(
        new_pipeline()
            .add(DieselMiddleware::<PgConnection>::new(datbase_url))
            .add(ImportServiceMiddleware::new(ImportService::new(
                library_uri.parse().unwrap(),
                store_uri.parse().unwrap(),
            )))
            .add(TokioMiddleware::new(handle))
            .build(),
    );
    let pipelines = finalize_pipeline_set(pipelines);
    let default_pipeline_chain = (default, ());

    // Router builder starts here
    trace!("finalize router");
    build_router(default_pipeline_chain, pipelines, |route| {
        route.get("/imports").to(ImportController::get_imports);
        route.post("/imports").to(ImportController::start_import);
        route
            .request(vec![Method::Put], "/imports/:id/raw")
            .with_path_extractor::<ImportRequestPath>()
            .to(ImportController::upload_raw_image);
        route
            .request(vec![Method::Put], "/imports/:id/sidecar")
            .with_path_extractor::<ImportRequestPath>()
            .to(ImportController::upload_sidecar);
        route
            .request(vec![Method::Put], "/imports/:id/image")
            .with_path_extractor::<ImportRequestPath>()
            .to(ImportController::upload_image);
        route
            .post("/imports/:id/finish")
            .with_path_extractor::<ImportRequestPath>()
            .to(ImportController::finish_upload);
    })
}
