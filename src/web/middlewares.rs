use std::io;
use std::sync::Arc;

use gotham;
use gotham::handler::HandlerFuture;
use gotham::middleware::{Middleware, NewMiddleware};
use gotham::state::State;

use service::ImportService;

pub struct ImportServiceMiddleware {
    service: Arc<ImportService>,
}

impl ImportServiceMiddleware {
    pub fn new() -> Self {
        ImportServiceMiddleware {
            service: Arc::new(ImportService::new()),
        }
    }
}

impl Middleware for ImportServiceMiddleware {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture> + 'static,
        Self: Sized,
    {
        state.put(ImportServiceMiddlewareData::new(self.service.clone()));

        chain(state)
    }
}

impl NewMiddleware for ImportServiceMiddleware {
    type Instance = ImportServiceMiddleware;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(ImportServiceMiddleware::new())
    }
}

#[derive(StateData)]
pub struct ImportServiceMiddlewareData {
    service: Arc<ImportService>,
}

impl ImportServiceMiddlewareData {
    pub fn new(service: Arc<ImportService>) -> Self {
        ImportServiceMiddlewareData { service: service }
    }

    pub fn service(&self) -> &ImportService {
        &self.service
    }
}
