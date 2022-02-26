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
impl TransactionType {
    pub fn to_string(&self) -> String {
        match &self {
            TransactionType::Deposit => "deposit".to_string(),
            TransactionType::Withdrawal => "withdrawal".to_string(),
            TransactionType::Dispute => "dispute".to_string(),
            TransactionType::Resolve => "resolve".to_string(),
            TransactionType::Chargeback => "chargeback".to_string(),
        }
    }
    pub fn from_string(transaction_type: &str) -> Result<TransactionType, String> {
        if transaction_type == "deposit" {
            return Ok(TransactionType::Deposit);
        }
        if transaction_type == "withdrawal" {
            return Ok(TransactionType::Withdrawal);
        }
        if transaction_type == "dispute" {
            return Ok(TransactionType::Dispute);
        }
        if transaction_type == "resolve" {
            return Ok(TransactionType::Resolve);
        }
        if transaction_type == "chargeback" {
            return Ok(TransactionType::Chargeback);
        }
        Err(format!("Invalid transaction type {}.", transaction_type))
    }
}

fn main() {
    println!("Hello, world!");
}
