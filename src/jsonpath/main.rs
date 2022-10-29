extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serde_json;

mod json_node;
mod json_path;
mod select_value;

use serde_json::Value;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("usage: json_path_tests <json> <path>");
        process::exit(1);
    }

    let json = &args[1];
    let json_path = &args[2];

    let query = json_path::compile(json_path);
    if let Err(e) = query {
        println!("Failed parsing json path, {}", e);
        process::exit(1);
    }
    let query = query.unwrap();
    let v = serde_json::from_str(json);
    if let Err(e) = v {
        println!("Failed parsing json, {}", e);
        process::exit(1);
    }
    let v: Value = v.unwrap();
    let path_calculator =
        json_path::PathCalculator::<json_path::DummyTrackerGenerator>::create(&query);
    let res = path_calculator.calc(&v);
    for r in res {
        println!("{}", r);
    }
}
