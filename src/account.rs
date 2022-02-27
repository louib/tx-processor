use rust_decimal::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashSet;

use crate::transaction::{Transaction, TransactionType};

pub struct Account {
    client_id: u16,

    available: Decimal,

    held: Decimal,

    locked: bool,

    // A cache of the transactions that were processed
    // for this account.
    transactions: BTreeMap<u32, Transaction>,

    // A cache of the IDs of the disputed transactions.
    pub disputed_transactions: HashSet<u32>,
}
impl Account {
    pub fn new(client_id: u16) -> Account {
        Account {
            client_id,
            held: Decimal::from_str("0.0").unwrap(),
            available: Decimal::from_str("0.0").unwrap(),
            locked: false,
            transactions: BTreeMap::new(),
            disputed_transactions: HashSet::new(),
        }
    }
    pub fn process_transaction(&mut self, tx: Transaction) {
        // Currently there is no way to process a transaction once the
        // account was locked.
        if self.locked {
            eprintln!("Could not process transaction: The account is locked.");
            return;
        }

        match tx.get_type() {
            TransactionType::Deposit => {
                if self.transactions.contains_key(&tx.transaction_id) {
                    return;
                }

                // TODO verify that this transaction was never processed?
                self.available += tx.amount.unwrap();
                self.transactions.insert(tx.transaction_id, tx);
            }
            TransactionType::Withdrawal => {
                if self.transactions.contains_key(&tx.transaction_id) {
                    return;
                }

                // TODO verify that this transaction was never processed?
                if self.available < tx.amount.unwrap() {
                    eprintln!("Could not process transaction: Insufficient amount.");
                    return;
                }

                self.available -= tx.amount.unwrap();
                self.transactions.insert(tx.transaction_id, tx);
            }
            TransactionType::Dispute => {
                let disputed_tx = match self.transactions.get(&tx.transaction_id) {
                    Some(tx) => tx,
                    None => return,
                };

                if !disputed_tx.is_disputable() {
                    eprintln!("Transaction {} is not disputable.", disputed_tx.transaction_id);
                    return;
                }

                self.available -= disputed_tx.amount.unwrap();
                self.held += disputed_tx.amount.unwrap();
                self.disputed_transactions.insert(tx.transaction_id.clone());
            }
            TransactionType::Resolve => {
                let disputed_tx = match self.transactions.get(&tx.transaction_id) {
                    Some(tx) => tx,
                    None => return,
                };

                if !self.disputed_transactions.contains(&tx.transaction_id) {
                    return;
                }

                self.available += disputed_tx.amount.unwrap();
                self.held -= disputed_tx.amount.unwrap();
                self.disputed_transactions.remove(&tx.transaction_id);
            }
            TransactionType::Chargeback => {
                let disputed_tx = match self.transactions.get(&tx.transaction_id) {
                    Some(tx) => tx,
                    None => return,
                };

                if !self.disputed_transactions.contains(&tx.transaction_id) {
                    return;
                }

                self.held -= disputed_tx.amount.unwrap();
                self.disputed_transactions.remove(&tx.transaction_id);
                self.locked = true;
            }
        };
    }

    pub fn get_total(&self) -> Decimal {
        self.available + self.held
    }

    pub fn print(&self) {
        println!(
            "{}, {}, {}, {}, {}",
            self.client_id,
            self.available
                .round_dp(crate::consts::DECIMAL_PRECISION)
                .normalize(),
            self.held.round_dp(crate::consts::DECIMAL_PRECISION).normalize(),
            self.get_total()
                .round_dp(crate::consts::DECIMAL_PRECISION)
                .normalize(),
            self.locked
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_deposit() {
        let mut account = Account::new(1);
        let mut tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Deposit,
            amount: Some(Decimal::from_str("100.0").unwrap()),
        };
        account.process_transaction(tx);
        assert_eq!(account.available, Decimal::from_str("100.0").unwrap());
    }

    #[test]
    pub fn test_duplicate_deposit() {
        let mut account = Account::new(1);
        let mut tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Deposit,
            amount: Some(Decimal::from_str("100.0").unwrap()),
        };
        account.process_transaction(tx.clone());
        // We will ignore a transaction that was already processed.
        account.process_transaction(tx);
        assert_eq!(account.available, Decimal::from_str("100.0").unwrap());
    }

    #[test]
    pub fn test_withdraw() {
        let mut account = Account::new(1);
        account.available = Decimal::from_str("100.0").unwrap();

        let mut tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Withdrawal,
            amount: Some(Decimal::from_str("50.0").unwrap()),
        };
        account.process_transaction(tx);
        assert_eq!(account.available, Decimal::from_str("50.0").unwrap());
    }

    #[test]
    pub fn test_duplicate_withdraw() {
        let mut account = Account::new(1);
        account.available = Decimal::from_str("100.0").unwrap();

        let mut tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Withdrawal,
            amount: Some(Decimal::from_str("50.0").unwrap()),
        };
        account.process_transaction(tx.clone());
        // We will ignore a transaction that was already processed.
        account.process_transaction(tx);
        assert_eq!(account.available, Decimal::from_str("50.0").unwrap());
    }

    #[test]
    pub fn test_withdraw_insufficient_funds() {
        let mut account = Account::new(1);
        account.available = Decimal::from_str("100.0").unwrap();

        let mut tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Withdrawal,
            amount: Some(Decimal::from_str("150.0").unwrap()),
        };
        account.process_transaction(tx);
        assert_eq!(account.available, Decimal::from_str("100.0").unwrap());
    }

    #[test]
    pub fn test_dispute() {
        let mut account = Account::new(1);

        let mut tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Deposit,
            amount: Some(Decimal::from_str("150.0").unwrap()),
        };
        let mut dispute_tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Dispute,
            amount: None,
        };
        account.process_transaction(tx);
        account.process_transaction(dispute_tx);
        assert_eq!(account.available, Decimal::from_str("0.0").unwrap());
        assert_eq!(account.held, Decimal::from_str("150.0").unwrap());
    }

    #[test]
    pub fn test_dispute_invalid_transaction() {
        let mut account = Account::new(1);

        let mut tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Deposit,
            amount: Some(Decimal::from_str("150.0").unwrap()),
        };
        let mut dispute_tx = Transaction {
            client_id: 1,
            transaction_id: 2,
            r#type: TransactionType::Dispute,
            amount: None,
        };
        account.process_transaction(tx);
        account.process_transaction(dispute_tx);
        assert_eq!(account.available, Decimal::from_str("150.0").unwrap());
        assert_eq!(account.held, Decimal::from_str("0.0").unwrap());
    }

    #[test]
    pub fn test_resolve() {
        let mut account = Account::new(1);

        let mut tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Deposit,
            amount: Some(Decimal::from_str("150.0").unwrap()),
        };
        let mut dispute_tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Dispute,
            amount: Some(Decimal::from_str("150.0").unwrap()),
        };
        let mut resolve_tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Resolve,
            amount: None,
        };
        account.process_transaction(tx);
        account.process_transaction(dispute_tx);
        account.process_transaction(resolve_tx);
        assert_eq!(account.available, Decimal::from_str("150.0").unwrap());
        assert_eq!(account.held, Decimal::from_str("0.0").unwrap());
    }

    #[test]
    pub fn test_resolve_invalid_transaction() {
        let mut account = Account::new(1);
        account.available = Decimal::from_str("100.0").unwrap();

        let mut resolve_tx = Transaction {
            client_id: 1,
            transaction_id: 7,
            r#type: TransactionType::Resolve,
            amount: None,
        };
        account.process_transaction(resolve_tx);
        assert_eq!(account.available, Decimal::from_str("100.0").unwrap());
        assert_eq!(account.held, Decimal::from_str("0.0").unwrap());
    }

    #[test]
    pub fn test_chargeback() {
        let mut account = Account::new(1);

        let mut tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Deposit,
            amount: Some(Decimal::from_str("150.0").unwrap()),
        };
        let mut dispute_tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Dispute,
            amount: None,
        };
        let mut chargeback_tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Chargeback,
            amount: None,
        };
        account.process_transaction(tx);
        account.process_transaction(dispute_tx);
        account.process_transaction(chargeback_tx);
        assert_eq!(account.available, Decimal::from_str("0.0").unwrap());
        assert_eq!(account.held, Decimal::from_str("0.0").unwrap());
        assert_eq!(account.locked, true);
    }

    #[test]
    pub fn test_chargeback_invalid_transaction() {
        let mut account = Account::new(1);
        account.available = Decimal::from_str("100.0").unwrap();

        let mut chargeback_tx = Transaction {
            client_id: 1,
            transaction_id: 1,
            r#type: TransactionType::Chargeback,
            amount: None,
        };
        account.process_transaction(chargeback_tx);
        assert_eq!(account.available, Decimal::from_str("100.0").unwrap());
        assert_eq!(account.held, Decimal::from_str("0.0").unwrap());
        assert_eq!(account.locked, false);
    }

    #[test]
    pub fn test_precision() {
        let mut account = Account::new(1);

        for i in 0..1_000_000 {
            let mut chargeback_tx = Transaction {
                client_id: 1,
                transaction_id: i,
                r#type: TransactionType::Deposit,
                amount: Some(Decimal::from_str("0.1").unwrap()),
            };
            account.process_transaction(chargeback_tx);
        }

        assert_eq!(account.available, Decimal::from_str("100000.0").unwrap());
    }
}
