use crate::api::BlockchainAPI;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Blockchain CLI")
        .version("1.0")
        .author("KOSASIH")
        .about("A command-line interface for the blockchain")
        .arg(Arg::with_name("storage_path")
             .long("storage-path")
             .takes_value(true)
             .required(true)
             .help("Path to the storage file"))
        .arg(Arg::with_name("create_block")
             .long("create-block")
             .takes_value(false)
             .help("Create a new block"))
        .arg(Arg::with_name("add_transaction")
             .long("add-transaction")
             .takes_value(true)
             .help("Add a new transaction"))
        .arg(Arg::with_name("get_state")
             .long("get-state")
             .takes_value(false)
             .help("Get the current blockchain state"))
        .get_matches();

    let storage_path = matches.value_of("storage_path").unwrap();
    let mut api = BlockchainAPI::new(storage_path);

    if let Some(_) = matches.value_of("create_block") {
        api.create_new_block(vec![]).unwrap();
        println!("New block created!");
    }

    if let Some(transaction) = matches.value_of("add_transaction") {
        let transaction: crate::transaction::Transaction = serde_json::from_str(transaction).unwrap();
        api.add_transaction(transaction).unwrap();
        println!("Transaction added!");
    }

    if let Some(_) = matches.value_of("get_state") {
        println!("Blockchain state: {:?}", api.get_blockchain_state());
    }
}
