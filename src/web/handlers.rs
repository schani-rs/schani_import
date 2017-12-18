use gotham::handler::HandlerFuture;
use gotham::state::State;

pub struct ImportController;

impl ImportController {
    pub fn get_imports(mut state: State) -> Box<HandlerFuture> {
        unimplemented!();
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
