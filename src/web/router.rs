use diesel::pg::PgConnection;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::router::route::dispatch::{finalize_pipeline_set, new_pipeline_set};
use gotham::middleware::pipeline::new_pipeline;
use gotham_middleware_diesel::DieselMiddleware;
use hyper::Method;

use super::extractors::ImportRequestPath;
use super::handlers::ImportController;
use super::middlewares::ImportServiceMiddleware;

pub fn build_app_router(datbase_url: &str) -> Router {
    trace!("build pipelines");
    let pipelines = new_pipeline_set();
    let (pipelines, default) = pipelines.add(
        new_pipeline()
            .add(DieselMiddleware::<PgConnection>::new(datbase_url))
            .add(ImportServiceMiddleware::new())
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
            .to(ImportController::start_import);
        route
            .request(vec![Method::Put], "/imports/:id/sidecar")
            .with_path_extractor::<ImportRequestPath>()
            .to(ImportController::start_import);
        route
            .request(vec![Method::Put], "/imports/:id/image")
            .with_path_extractor::<ImportRequestPath>()
            .to(ImportController::start_import);
        route
            .post("/imports/:id/finish")
            .with_path_extractor::<ImportRequestPath>()
            .to(ImportController::start_import);
    })
}