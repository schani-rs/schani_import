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

#[derive(Deserialize)]
struct NewImportRequestBody {
    title: Option<String>,
    user_id: i32,
}

impl Into<NewImport> for NewImportRequestBody {
    fn into(self) -> NewImport {
        NewImport {
            title: self.title,
            user_id: self.user_id,
        }
    }
}

pub struct ImportController;

impl ImportController {
    pub fn get_imports(state: State) -> Box<HandlerFuture> {
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
        let f = Body::take_from(&mut state)
            .concat2()
            .then(move |raw_body| match raw_body {
                Ok(json_chunk) => {
                    let bytes = json_chunk.to_vec();
                    let json = String::from_utf8(bytes).unwrap();
                    let body: NewImportRequestBody = serde_json::from_str(json.as_str()).unwrap();
                    let new_import = body.into();

                    let import = {
                        let image_service: &ImportServiceMiddlewareData =
                            state.borrow::<ImportServiceMiddlewareData>();
                        let conn = connection(&state);

                        image_service.service().create_import(&conn, new_import)
                    };

                    let json = serde_json::to_string(&import).unwrap();

                    let resp = create_response(
                        &state,
                        StatusCode::Ok,
                        Some((json.into_bytes(), mime::APPLICATION_JSON)),
                    );
                    future::ok((state, resp))
                }
                Err(e) => future::err((state, e.into_handler_error())),
            });

        Box::new(f)
    }

    pub fn upload_raw_image(mut state: State) -> Box<HandlerFuture> {
        let f = Body::take_from(&mut state)
            .concat2()
            .then(move |raw_body| match raw_body {
                Ok(binary_chunk) => {
                    let raw_bytes = binary_chunk.to_vec();
                    let id = ImportRequestPath::borrow_from(&state).id();

                    let import = {
                        let image_service: &ImportServiceMiddlewareData =
                            state.borrow::<ImportServiceMiddlewareData>();
                        let conn = connection(&state);

                        image_service
                            .service()
                            .add_raw_file(&conn, id, raw_bytes.as_slice())
                    };

                    let json = serde_json::to_string(&import).unwrap();

                    let resp = create_response(
                        &state,
                        StatusCode::Ok,
                        Some((json.into_bytes(), mime::APPLICATION_JSON)),
                    );
                    future::ok((state, resp))
                }
                Err(e) => future::err((state, e.into_handler_error())),
            });

        Box::new(f)
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
