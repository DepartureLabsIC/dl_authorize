use dl_authorize::request::RequestResourceBuilder;
use dl_authorize::statement::StatementResource;
use dl_authorize::*;
use std::collections::HashMap;

pub fn main() {
    let resource = StatementResource::Resource("canister".to_string()).add_nested(
        StatementResource::Resource("key_store".to_string()).add_nested_resources(vec![]),
    );
}

pub struct MutateRequest {
    action: Action,
    user: String,
    caller: String,
}

pub enum Action {
    Insert { k: String, v: String },
    Delete(String),
}

pub struct UserStore {
    data: HashMap<String, KVStore>,
}

pub struct KVStore {
    data: HashMap<String, String>,
}
