use serde::{Deserialize, Deserializer};

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

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<TransactionType>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buf = String::deserialize(deserializer)?;

        match TransactionType::from_string(&buf) {
            Ok(b) => Ok(Some(b)),
            Err(e) => Err(e).map_err(serde::de::Error::custom),
        }
    }
}
