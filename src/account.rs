use crate::transaction::{Transaction, TransactionType};

pub struct Account {
    client_id: u16,

    available: f64,

    held: f64,

    total: f64,

    locked: bool,
}
impl Account {
    pub fn process_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        match tx.get_type() {
            TransactionType::Deposit => {
                // TODO verify that this transaction was never processed?
                self.available += tx.amount as f64;
            }
            TransactionType::Withdrawal => {}
            TransactionType::Dispute => {}
            TransactionType::Resolve => {}
            TransactionType::Chargeback => {}
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_deposit_into_account() {
        let mut account = Account {
            client_id: 1,
            held: 0.0,
            available: 0.0,
            total: 0.0,
            locked: false,
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
}
