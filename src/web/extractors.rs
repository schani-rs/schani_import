use gotham;
use hyper;

#[derive(StateData, PathExtractor, StaticResponseExtender)]
pub struct ImportRequestPath {
    id: i32,
}

impl ImportRequestPath {
    pub fn id(&self) -> i32 {
        self.id
    }
}
