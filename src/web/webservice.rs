use fern;
use futures::{self, Future, Stream};
use gotham::handler::NewHandlerService;
use hyper::server::Http;
use tokio_core::reactor::Core;

use std::io;

use super::router::build_app_router;

use log::LevelFilter;

pub struct ImportWebService<'a> {
    database_url: &'a str,
}

impl<'a> ImportWebService<'a> {
    pub fn new(database_url: &'a str) -> Self {
        ImportWebService {
            database_url: database_url,
        }
    }

    fn set_logging(&self) {
        fern::Dispatch::new()
            .level(LevelFilter::Info)
            .chain(io::stdout())
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{}][{}]{}",
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .apply()
            .unwrap();
    }

    pub fn run(self) {
        self.set_logging();

        let mut core = Core::new().unwrap();
        let addr = "0.0.0.0:8001".parse().unwrap();
        trace!("create router");
        let router = build_app_router(
            self.database_url,
            "http://localhost:8000",
            "http://localhost:8002",
            core.remote(),
        );
        trace!("create server");
        let handle = core.handle();
        let server = Http::new()
            .serve_addr_handle(&addr, &handle, NewHandlerService::new(router))
            .unwrap();

        info!("server listening on 0.0.0.0:8001");
        let handle2 = handle.clone();
        handle.spawn(
            server
                .for_each(move |conn| {
                    handle2.spawn(
                        conn.map(|_| ())
                            .map_err(|err| println!("server error: {:?}", err)),
                    );
                    Ok(())
                })
                .map_err(|_| ()),
        );

        core.run(futures::future::empty::<(), ()>()).unwrap();
    }
}
