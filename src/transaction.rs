use serde::{Deserialize, Deserializer};

#[derive(Debug)]
#[derive(Deserialize)]
struct Transaction {
    #[serde(deserialize_with = "crate::transaction::TransactionType::deserialize")]
    r#type: TransactionType,
    // TODO rename client
    client_id: u16,
    // TODO rename tx
    transaction_id: u32,
    amount: f32,
}

#[derive(Debug)]
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

    pub fn deserialize<'de, D>(deserializer: D) -> Result<TransactionType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buf = String::deserialize(deserializer)?;

        match TransactionType::from_string(&buf) {
            Ok(b) => Ok(b),
            Err(e) => Err(e).map_err(serde::de::Error::custom),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_deposit_transaction() {
    }
}
