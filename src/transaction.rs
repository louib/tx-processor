use rust_decimal::prelude::*;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Transaction {
    #[serde(deserialize_with = "crate::transaction::TransactionType::deserialize")]
    pub r#type: TransactionType,

    #[serde(rename = "client")]
    pub client_id: u16,

    #[serde(rename = "tx")]
    pub transaction_id: u32,

    pub amount: Option<Decimal>,
}
impl Transaction {
    pub fn get_type(&self) -> &TransactionType {
        &self.r#type
    }
    pub fn is_disputable(&self) -> bool {
        return [TransactionType::Deposit, TransactionType::Withdrawal].contains(self.get_type());
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}
impl TransactionType {
    // TODO this boilerplate code could be replaced by using a macro like
    // https://docs.rs/strum_macros/0.24.0/strum_macros/derive.EnumString.html,
    // but I'm concerned about supply-chain attacks using Rust macros, so
    // I'd like to review the crate before using it.
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
        let serialized_tx: &str = "deposit,1,1,1.0";
        let tx: Transaction = deserialize_single_transaction(serialized_tx).unwrap();
        assert_eq!(*tx.get_type(), TransactionType::Deposit);
        assert_eq!(tx.client_id, 1);
        assert_eq!(tx.transaction_id, 1);
        assert_eq!(tx.amount, Some(Decimal::from_str("1.0").unwrap()));
    }

    #[test]
    #[should_panic]
    pub fn test_parse_invalid_transaction_type() {
        let serialized_tx: &str = "invalid,1,1,1.0";
        deserialize_single_transaction(serialized_tx);
    }

    #[test]
    pub fn test_parse_with_spaces() {
        let serialized_tx: &str = "deposit, 1, 1, 1.0";
        let tx: Transaction = deserialize_single_transaction(serialized_tx).unwrap();
    }

    #[test]
    pub fn test_parse_no_amount() {
        let serialized_tx: &str = "chargeback, 1, 1,";
        let tx: Transaction = deserialize_single_transaction(serialized_tx).unwrap();
        assert_eq!(*tx.get_type(), TransactionType::Chargeback);
        assert_eq!(tx.amount, None);
        assert_eq!(tx.client_id, 1);
    }

    #[test]
    pub fn test_parse_decimals() {
        let serialized_tx: &str = "withdrawal, 1, 1, 3.5545";
        let tx: Transaction = deserialize_single_transaction(serialized_tx).unwrap();
        assert_eq!(*tx.get_type(), TransactionType::Withdrawal);
        assert_eq!(tx.amount, Some(Decimal::from_str("3.5545").unwrap()));
    }

    pub fn deserialize_single_transaction(serialized_tx: &str) -> Result<Transaction, String> {
        let header: &str = "type,client,tx,amount";
        let csv_file: String = format!("{}\n{}", header, serialized_tx);

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .trim(csv::Trim::All)
            .from_reader(csv_file.as_bytes());
        for result in reader.deserialize() {
            let tx: Transaction = result.expect("Could not deserialize transaction.");
            return Ok(tx);
        }
        return Err("Did not deserialize any transaction".to_string());
    }
}
