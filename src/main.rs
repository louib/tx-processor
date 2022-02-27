use std::env;
use std::error::Error;
use std::io;

use csv;

mod account;
mod transaction;

use transaction::Transaction;

fn process_transactions(transactions_file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(transactions_file_path)?;
    for result in reader.deserialize() {
        let tx: Transaction = result.expect("Could not deserialize transaction.");
        // TODO add the account if it doesn't exist
        // TODO process the transactions
        println!("{:?}", tx);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        panic!("You must provide the path to a transaction file to process.");
    }
    let transactions_file_path = &args[1];

    if let Err(err) = process_transactions(transactions_file_path) {
        panic!("Error while processing the transactions: {}", err);
    }

    println!("Hello, world!");
}
