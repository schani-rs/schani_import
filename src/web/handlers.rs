use std::convert::Into;

use futures::{future, Future, Stream};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham_middleware_diesel::state_data::connection;
use hyper::{Body, StatusCode};
use mime;
use serde_json;

use models::{Import, NewImport};
use super::extractors::ImportRequestPath;
use super::middlewares::ImportServiceMiddlewareData;

pub struct ImportController;

impl ImportController {
    pub fn get_imports(mut state: State) -> Box<HandlerFuture> {
        let imports = {
            let import_service: &ImportServiceMiddlewareData =
                state.borrow::<ImportServiceMiddlewareData>();
            let conn = connection(&state);

            import_service.service().get_imports(&conn)
        };

        let json = serde_json::to_string(&imports).unwrap();

        let resp = create_response(
            &state,
            StatusCode::Ok,
            Some((json.into_bytes(), mime::APPLICATION_JSON)),
        );
        Box::new(future::ok((state, resp)))
    }

    pub fn start_import(mut state: State) -> Box<HandlerFuture> {
        unimplemented!();
    }

    pub fn upload_raw_image(mut state: State) -> Box<HandlerFuture> {
        unimplemented!();
    }

    pub fn upload_sidecar(mut state: State) -> Box<HandlerFuture> {
        unimplemented!();
    }

    pub fn upload_image(mut state: State) -> Box<HandlerFuture> {
        unimplemented!();
    }

    pub fn finish_upload(mut state: State) -> Box<HandlerFuture> {
        unimplemented!();
    }
}
