#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[ink::contract]
mod amail {
    use std::collections::BTreeMap;
    use chrono;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Mail {
        sent: BTreeMap<AccountId, Vec<String>>,
        received: BTreeMap<AccountId, Vec<String>>,
        timestamps: BTreeMap<String, i64>,
        hashes: BTreeMap<String, Vec<String>>,
        masks: BTreeMap<String, String>,
        contacts: BTreeMap<AccountId, Vec<AccountId>>,
    }

    impl Mail {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                sent : BTreeMap::new(),
                received : BTreeMap::new(),
                timestamps: BTreeMap::new(),
                hashes: BTreeMap::new(),
                masks: BTreeMap::new(),
                contacts: BTreeMap::new(),
            }
        }

        

        

        #[ink(message)]
        pub fn get_received_mail(&self) -> Vec<String> {
            let caller = self.env().caller();
            let mail_list_op = self.received.get(&caller);
            if mail_list_op.is_none() {
                return Vec::new();
            }
            else {
                return mail_list_op.unwrap().clone().to_vec();
            }
        }

        #[ink(message)]
        pub fn get_sent_mail(&self) -> Vec<String> {
            let caller = self.env().caller();
            let mail_list_op = self.sent.get(&caller);
            if mail_list_op.is_none() {
                return Vec::new();
            }
            else {
                return mail_list_op.unwrap().clone().to_vec();
            }
        }

        #[ink(message)]
        pub fn get_contacts(&self) -> Vec<AccountId> {
            let caller = self.env().caller();
            let contact_list_op = self.contacts.get(&caller);
            if contact_list_op.is_none() {
                return Vec::new();
            }
            else {
                return contact_list_op.unwrap().clone().to_vec();
            }
        }

        #[ink(message)]
        pub fn add_contacts(&mut self, add: AccountId) -> bool {
            let caller = self.env().caller();
            let contact_list_op = self.contacts.get(&caller);
            if contact_list_op.is_none() {
                return false;
            }
            else {
                let mut contact_list = contact_list_op.unwrap().clone().to_vec();
                contact_list.push(add);
                self.contacts.remove(&caller);
                self.contacts.insert(caller.clone(), contact_list);
                return true;
            }
        }

        #[ink(message)]
        pub fn send_mail(&mut self, to: AccountId, mail_id: String, hash1: String, hash2: String, hash3: String, phrase: String) -> bool {
            let caller = self.env().caller();
            let now = chrono::offset::Utc::now().timestamp();
            let hash_list = vec![hash1, hash2, hash3];
            let mut sent_list: Vec<String>;
            let mut received_list: Vec<String>;
            if self.timestamps.get(&mail_id).is_some() || self.hashes.get(&mail_id).is_some() || self.masks.get(&mail_id).is_some() {
                return false;
            }

            if self.sent.get(&caller).is_none() {
                sent_list =  Vec::new();
                sent_list.push(mail_id.clone());
                
            }
            else {
                sent_list = self.sent.get(&caller).unwrap().clone().to_vec();
                sent_list.push(mail_id.clone());
            }
            if self.received.get(&to).is_none() {
                received_list =  Vec::new();
                received_list.push(mail_id.clone());
                
            }
            else {
                received_list = self.received.get(&to).unwrap().clone().to_vec();
                received_list.push(mail_id.clone());
            }
            self.sent.remove(&caller);
            self.sent.insert(caller.clone(), sent_list);
            self.received.remove(&to);
            self.received.insert(to.clone(), received_list);
            self.timestamps.insert(mail_id.clone(), now);
            self.hashes.insert(mail_id.clone(), hash_list);
            let m1 = now.to_string();
            let (m2, m3) = phrase.split_at(10);
            return true;


        }

        

        #[ink(message, payable)]
        pub fn tip_contact(&mut self, ctct: AccountId, amt: u128) -> bool {
            assert!(self.env().transferred_value() == amt);
            assert!(amt <= self.env().balance(), "insufficient funds!");
            if self.env().transfer(ctct, amt).is_err() {
                panic!(
                    "failed to transfer funds to the contact. this happens when the contract balance\
                    falls below the minimum threshold. try again later." 
                )
            }
            return true;

            
        }

    }



    
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        //use chrono;
        
        #[ink::test]
        fn test_init() {
            let ml = Mail::new();
            let sent = ml.get_sent_mail(); //fetching all fund requests
            assert_eq!(sent.len(), 0);           
        }

        #[ink::test]
        fn test_tip() {
            let accounts = default_accounts();
            let mut ml = Mail::new();
            set_sender(accounts.bob); 
            set_balance(accounts.bob, 10000);
            set_balance(accounts.eve, 100);
            let _res = ink::env::pay_with_call!(ml.tip_contact(accounts.eve, 2000), 2000);
            let bal_bob = get_balance(accounts.bob);
            let bal_eve = get_balance(accounts.eve);
            assert_eq!(bal_bob, 8000);
            assert_eq!(bal_eve, 2100);           
        }

        


         /// Returns the `contract_instance`.
         fn create_contract(initial_balance: Balance) -> Mail {
            let accounts = default_accounts();
            set_sender(accounts.alice);
            set_balance(contract_id(), initial_balance);
            Mail::new()
        }

        fn contract_id() -> AccountId {
            ink::env::test::callee::<ink::env::DefaultEnvironment>()
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
        }

        fn default_accounts(
        ) -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(
                account_id, balance,
            )
        }

        fn get_balance(account_id: AccountId) -> Balance {
            ink::env::test::get_account_balance::<ink::env::DefaultEnvironment>(
                account_id,
            )
            .expect("Cannot get account balance")
        }
    


    }
}
