use std::env;

mod account;
mod bank;
mod transaction;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You must provide the path to a transaction file to process.");
    }
    let transactions_file_path = &args[1];

    let mut bank = bank::Bank::new();

    if let Err(err) = bank.process_transactions(transactions_file_path) {
        panic!("Error while processing the transactions: {}", err);
    }

    bank.print()
}
