extern crate valico;
extern crate "rustc-serialize" as serialize;

use serialize::json::{Json, ToJson};
use valico::{Builder, MutableJson};

fn main() {

    let params = Builder::build(|params| {
        params.req_nested("user", Builder::list(), |params| {
            params.req_typed("name", Builder::string());
            params.req_typed("friend_ids", Builder::list_of(Builder::u64()))
        });
    });

    let mut obj = r#"{"user": {"name": "Frodo", "friend_ids": ["1223"]}}"#.parse::<Json>().unwrap();

    match params.process(obj.as_object_mut().unwrap()) {
        Ok(()) => {
            println!("Result object is {}", obj.to_pretty_str());
        },
        Err(err) => {
            panic!("Error during process: {}", err.to_json().to_pretty_str());
        }
    }

}