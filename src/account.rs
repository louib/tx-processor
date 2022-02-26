use std::collections::BTreeMap;

use crate::transaction::{Transaction, TransactionType};

pub struct Account {
    client_id: u16,

    available: f64,

    held: f64,

    locked: bool,

    // A cache of the transactions that were processed
    // for this account.
    pub transactions: BTreeMap<String, Transaction>,
}
impl Account {
    pub fn process_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        match tx.get_type() {
            TransactionType::Deposit => {
                // TODO verify that this transaction was never processed?
                self.available += tx.amount as f64;
            }
            TransactionType::Withdrawal => {
                if self.available < tx.amount as f64 {
                    return Err("Could not process transaction: Insufficient amount.".to_string());
                }
                self.available -= tx.amount as f64;
            }
            TransactionType::Dispute => {


            }
            TransactionType::Resolve => {}
            TransactionType::Chargeback => {}
        };
        Ok(())
    }
    pub fn get_total(&self) -> f64 {
        self.available + self.held
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_deposit() {
        let mut account = Account {
            client_id: 1,
            held: 0.0,
            available: 0.0,
            locked: false,
            transactions: BTreeMap::new(),
        };
        let mut tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Deposit,
            amount: 100.0,
        };
        account.process_transaction(tx).unwrap();
        assert_eq!(account.available, 100.0);
    }

    #[test]
    pub fn test_withdraw() {
        let mut account = Account {
            client_id: 1,
            held: 0.0,
            available: 100.0,
            locked: false,
            transactions: BTreeMap::new(),
        };
        let mut tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Withdrawal,
            amount: 50.0,
        };
        account.process_transaction(tx).unwrap();
        assert_eq!(account.available, 50.0);
    }

    #[test]
    pub fn test_withdraw_insufficient_funds() {
        let mut account = Account {
            client_id: 1,
            held: 0.0,
            available: 100.0,
            locked: false,
            transactions: BTreeMap::new(),
        };
        let mut tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Withdrawal,
            amount: 150.0,
        };
        if let Ok(()) = account.process_transaction(tx) {
            panic!("Should not allow to withdraw more funds than available.");
        }
        assert_eq!(account.available, 100.0);
    }
}
