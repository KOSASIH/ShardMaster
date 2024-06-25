// shard_master_cli.rs
use reqwest::blocking::get;
use serde_json::{Map, Value};

fn get_shards() -> Value {
    let url = "http://localhost:8080/shards";
    let response = get(url).unwrap();
    response.json().unwrap()
}

fn get_shard(shard_id: u32) -> Value {
    let url = format!("http://localhost:8080/shards/{}", shard_id);
    let response = get(url).unwrap();
    response.json().unwrap()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <command> [shard_id]", args[0]);
        return;
    }
    let command = &args[1];
    match command.as_str() {
        "get-shards" => {
            let shards = get_shards();
            println!("{}", serde_json::to_string_pretty(&shards).unwrap());
        }
        "get-shard" => {
            if args.len() < 3 {
                println!("Usage: {} <command> <shard_id>", args[0]);
                return;
            }
            let shard_id: u32 = args[2].parse().unwrap();
            let shard = get_shard(shard_id);
            println!("{}", serde_json::to_string_pretty(&shard).unwrap());
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
