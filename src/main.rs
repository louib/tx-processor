use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Transaction {
    r#type: String,
    // TODO rename client
    client_id: u16,
    // TODO rename tx
    transaction_id: u32,
    amount: f32,
}

enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

fn main() {
    println!("Hello, world!");
}
