use crate::transaction::{Transaction, TransactionType};

pub struct Account {
    client_id: u16,

    available: f64,

    held: f64,

    total: f64,

    locked: bool,
}
impl Account {
    pub fn process_transaction(&mut self, tx: Transaction) {
        match tx.get_type() {
            TransactionType::Deposit => {}
            TransactionType::Withdrawal => {}
            TransactionType::Dispute => {}
            TransactionType::Resolve => {}
            TransactionType::Chargeback => {}
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_deposit_into_account() {}
}
