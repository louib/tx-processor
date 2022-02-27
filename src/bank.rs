use std::collections::BTreeMap;

use crate::account::Account;

pub struct Bank {
    /// The accounts handled by the bank, indexed
    /// by customer ID.
    pub accounts: BTreeMap<u16, Transaction>,
}
impl Bank {
    pub fn process_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        // TODO create the account if it does not exist.
        // TODO process the transaction
    }
    pub fn print(&self) {

    }
}
