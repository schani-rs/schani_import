use std::convert::Into;

use futures::{future, Future, Stream};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham_middleware_diesel::state_data::connection;
use gotham_middleware_tokio::TokioMiddlewareData;
use hyper::{Body, StatusCode};
use mime;
use serde_json;

use models::NewImport;
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
                    let id = ImportRequestPath::borrow_from(&state).id();

                    let save_raw_image = {
                        let image_service: &ImportServiceMiddlewareData =
                            state.borrow::<ImportServiceMiddlewareData>();
                        let handle = state
                            .borrow::<TokioMiddlewareData>()
                            .handle()
                            .handle()
                            .expect("got no handle from remote");
                        let conn = connection(&state);

                        let service = image_service.service().clone();
                        image_service
                            .service()
                            .add_raw_file(&handle, binary_chunk.to_vec())
                            .and_then(move |raw_image_id| {
                                info!("uploaded raw image {}", raw_image_id);
                                Ok(service.save_raw_image_id(&conn, id, raw_image_id))
                            })
                    };

                    let b: Box<HandlerFuture> = Box::new(
                        save_raw_image
                            .and_then(|import| {
                                info!("raw image for import {} saved", import.id);
                                let json = serde_json::to_string(&import).unwrap();

                                let resp = create_response(
                                    &state,
                                    StatusCode::Ok,
                                    Some((json.into_bytes(), mime::APPLICATION_JSON)),
                                );
                                future::ok((state, resp))
                            })
                            .map_err(|e| panic!(e)),
                    );
                    b
                }
                Err(e) => Box::new(future::err((state, e.into_handler_error()))),
            });

        Box::new(f)
    }

    pub fn upload_sidecar(mut state: State) -> Box<HandlerFuture> {
        let f = Body::take_from(&mut state)
            .concat2()
            .then(move |raw_body| match raw_body {
                Ok(binary_chunk) => {
                    let sidecar_bytes = binary_chunk.to_vec();
                    let id = ImportRequestPath::borrow_from(&state).id();

                    let importing = {
                        let image_service: &ImportServiceMiddlewareData =
                            state.borrow::<ImportServiceMiddlewareData>();
                        let handle = state
                            .borrow::<TokioMiddlewareData>()
                            .handle()
                            .handle()
                            .expect("got no handle from remote");
                        let conn = connection(&state);

                        let service = image_service.service().clone();
                        image_service
                            .service()
                            .add_sidecar(&handle, sidecar_bytes)
                            .and_then(move |sidecar_id| {
                                info!("uploaded sidecar {}", sidecar_id);
                                Ok(service.save_sidecar_id(&conn, id, sidecar_id))
                            })
                    };

                    let b: Box<HandlerFuture> = Box::new(
                        importing
                            .and_then(|import| Ok(serde_json::to_string(&import).unwrap()))
                            .and_then(|json| {
                                let resp = create_response(
                                    &state,
                                    StatusCode::Ok,
                                    Some((json.into_bytes(), mime::APPLICATION_JSON)),
                                );
                                future::ok((state, resp))
                            })
                            .map_err(|_| unimplemented!()),
                    );
                    b
                }
                Err(e) => Box::new(future::err((state, e.into_handler_error()))),
            });

        Box::new(f)
    }

    pub fn upload_image(mut state: State) -> Box<HandlerFuture> {
        let f = Body::take_from(&mut state)
            .concat2()
            .then(move |raw_body| match raw_body {
                Ok(binary_chunk) => {
                    let sidecar_bytes = binary_chunk.to_vec();
                    let id = ImportRequestPath::borrow_from(&state).id();

                    let importing = {
                        let image_service: &ImportServiceMiddlewareData =
                            state.borrow::<ImportServiceMiddlewareData>();
                        let handle = state
                            .borrow::<TokioMiddlewareData>()
                            .handle()
                            .handle()
                            .expect("got no handle from remote");
                        let conn = connection(&state);

                        let service = image_service.service().clone();
                        image_service
                            .service()
                            .add_image(&handle, sidecar_bytes)
                            .and_then(move |sidecar_id| {
                                info!("uploaded image {}", sidecar_id);
                                Ok(service.save_image_id(&conn, id, sidecar_id))
                            })
                    };

                    let b: Box<HandlerFuture> = Box::new(
                        importing
                            .and_then(|import| Ok(serde_json::to_string(&import).unwrap()))
                            .and_then(|json| {
                                let resp = create_response(
                                    &state,
                                    StatusCode::Ok,
                                    Some((json.into_bytes(), mime::APPLICATION_JSON)),
                                );
                                future::ok((state, resp))
                            })
                            .map_err(|_| unimplemented!()),
                    );
                    b
                }
                Err(e) => Box::new(future::err((state, e.into_handler_error()))),
            });

        Box::new(f)
    }

    pub fn finish_upload(state: State) -> Box<HandlerFuture> {
        let id = ImportRequestPath::borrow_from(&state).id();
        let imports = {
            let import_service: &ImportServiceMiddlewareData =
                state.borrow::<ImportServiceMiddlewareData>();
            let conn = connection(&state);

            import_service.service().finish_import(&conn, id)
        };

        let json = serde_json::to_string(&imports).unwrap();

        let resp = create_response(
            &state,
            StatusCode::Ok,
            Some((json.into_bytes(), mime::APPLICATION_JSON)),
        );
        Box::new(future::ok((state, resp)))
    }
}
