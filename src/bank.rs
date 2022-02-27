use std::collections::BTreeMap;

use crate::account::Account;
use crate::transaction::Transaction;

pub struct Bank {
    /// The accounts handled by the bank, indexed
    /// by customer ID.
    pub accounts: BTreeMap<u16, Account>,
}
impl Bank {
    pub fn process_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        // Creating a new account if it doesn't exist could be made more efficient by
        // using the BTreeMap::try_insert function, so that only one search is performed
        // on the B-Tree. This feature is still experimental so I decided not to use it
        // at the moment.
        let account: &mut Account = match self.accounts.get_mut(&tx.client_id) {
            Some(a) => a,
            None => {
                self.accounts
                    .insert(tx.client_id, Account::new(tx.client_id))
                    .unwrap();
                self.accounts.get_mut(&tx.client_id).unwrap()
            }
        };
        account.process_transaction(tx)?;
        Ok(())
    }
    pub fn print(&self) {}
}
