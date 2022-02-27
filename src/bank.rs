use std::collections::BTreeMap;
use std::error::Error;
use std::io;

use crate::account::Account;
use crate::transaction::Transaction;

pub struct Bank {
    /// The accounts handled by the bank, indexed
    /// by customer ID.
    pub accounts: BTreeMap<u16, Account>,
}
impl Bank {
    pub fn new() -> Bank {
        Bank {
            accounts: BTreeMap::new(),
        }
    }
    pub fn process_transactions(&mut self, transactions_file_path: &str) -> Result<(), Box<dyn Error>> {
        let mut reader = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .has_headers(true)
            .from_path(transactions_file_path)?;
        for result in reader.deserialize() {
            let tx: Transaction = result.expect("Could not deserialize transaction.");
            self.process_transaction(tx);
        }
        Ok(())
    }

    pub fn process_transaction(&mut self, tx: Transaction) {
        // Creating a new account if it doesn't exist could be made more efficient by
        // using the BTreeMap::try_insert function, so that only one search is performed
        // on the B-Tree. This feature is still experimental so I decided not to use it
        // at the moment.
        let account: &mut Account = match self.accounts.get_mut(&tx.client_id) {
            Some(a) => a,
            None => {
                self.accounts.insert(tx.client_id, Account::new(tx.client_id));
                self.accounts.get_mut(&tx.client_id).unwrap()
            }
        };
        account.process_transaction(tx);
    }
    pub fn print(&self) {}
}
