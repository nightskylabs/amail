#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[ink::contract]
mod amail {
    use ink::storage::Mapping;
    use ink::prelude::{vec, vec::Vec};
    use ink::prelude::string::{String, ToString};
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Mail {
        code: String,
        sent: Mapping<AccountId, Vec<String>>,
        received: Mapping<AccountId, Vec<String>>,
        timestamps: Mapping<String, u64>,
        algos: Mapping<String, Vec<u8>>,
        masks: Mapping<String, String>,
        contacts: Mapping<AccountId, Vec<AccountId>>,
    }

    impl Mail {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                code: "nonceecnon".to_string(),
                sent : Mapping::default(),
                received : Mapping::default(),
                timestamps: Mapping::default(),
                algos: Mapping::default(),
                masks: Mapping::default(),
                contacts: Mapping::default(),
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
        pub fn get_algos_and_mask(&self, mail_id: String) -> (String, Vec<u8>) {
            let caller = self.env().caller();
            let received_op = self.received.get(&caller);
            let sent_op = self.sent.get(&caller);
            let mask_op = self.masks.get(&mail_id);
            let algos_op = self.algos.get(&mail_id);
            if mask_op.is_none() || algos_op.is_none(){
                assert!(false, "this mail might not exist");
                return ("".to_string(), vec![3,3,3]);
            }

            if sent_op.is_none() && received_op.is_none() {
                assert!(false, "something went wrong");
                return ("".to_string(), vec![3,3,3]);
            }
            else {
                let mut flag = 0;
                if sent_op.is_some() {
                    let sent_list = sent_op.unwrap().to_vec();
                    for item in sent_list {
                        if item == mail_id {
                            flag = 1;
                            break;
                        }
                    }
                }
                if flag != 1 && received_op.is_some() {
                    let received_list = received_op.unwrap().to_vec();
                    for item in received_list {
                        if item == mail_id {
                            flag = 1;
                            break;
                        }
                    }
                }
                
        
                if flag != 1 {
                    assert!(false, "you neither sent or received this mail");
                    return ("".to_string(), vec![3,3,3]);
                }
                else {
                    let mask = mask_op.unwrap().clone();
                    let algos = algos_op.unwrap().clone().to_vec();
                    return (mask, algos);
                }
            }
        }

        #[ink(message)]
        pub fn add_contacts(&mut self, add: AccountId) -> bool {
            let caller = self.env().caller();
            let contact_list_op = self.contacts.get(&caller);
            if contact_list_op.is_none() {
                let mut contact_list = Vec::new();
                contact_list.push(add);
                self.contacts.insert(caller.clone(), &contact_list);
                return true;
            }
            else {
                let mut contact_list = contact_list_op.unwrap().clone().to_vec();
                contact_list.push(add);
                self.contacts.remove(&caller);
                self.contacts.insert(caller.clone(), &contact_list);
                return true;
            }
        }

        #[ink(message)]
        pub fn send_mail(&mut self, to: AccountId, mail_id: String, phrase: String) -> bool {
            let caller = self.env().caller();
            
            let mut sent_list: Vec<String>;
            let mut received_list: Vec<String>;
            if self.timestamps.get(&mail_id).is_some() || self.algos.get(&mail_id).is_some() || self.masks.get(&mail_id).is_some() {
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

            let algo_gen = phrase.as_bytes();
            let algo1 = algo_gen[0]%3;
            let algo2 = algo_gen[algo_gen.len()/2]%3;
            let algo3 = algo_gen[algo_gen.len() - 1]%3;
            let n1 = algo_gen[1];
            let n2 = algo_gen[algo_gen.len()/2 + 1];
            let n3 = algo_gen[algo_gen.len() - 2];
            let algo_list = vec![algo1, algo2, algo3];

            self.sent.remove(&caller);
            self.sent.insert(caller.clone(), &sent_list);
            self.received.remove(&to);
            self.received.insert(to.clone(), &received_list);

            let now = self.env().block_timestamp();
            self.timestamps.insert(mail_id.clone(), &now);
            self.algos.insert(mail_id.clone(), &algo_list);
            
            let rs = &self.code;
            
            let now_st = now.clone().to_string();
            let split1 = n1%(now_st.clone().len() as u8);
            let split2 = n2%(phrase.len() as u8);
            let split3 = n3%(rs.len() as u8);
            let binding = now_st.clone();
            let (m1, m2) = binding.split_at(split1.into());
            let (m3, m4) = phrase.split_at(split2.into());
            let (m5, m6) = rs.split_at(split3.into());
            
           
            let mut m11 = String::from(m1);
            let m21 = String::from(m2);
            let m31 = String::from(m3);
            let m41 = String::from(m4);
            let m51 = String::from(m5);
            let m61 = String::from(m6);
            
            m11.push_str(&m31);
            m11.push_str(&m51);
            m11.push_str(&m21);
            m11.push_str(&m61);
            m11.push_str(&m41);

            self.masks.insert(mail_id.clone(), &m11);

            let (_x, new_code1) = rs.split_at(rs.len()/2);
            let mut m71 = String::from(new_code1);
            m71.push_str(&m41);
            m71.retain(|c| !c.is_whitespace());
            if m71.len() > 50 {
                let (_y, new_m71) = m71.split_at(49);
                self.code = String::from(new_m71);
            }
            else {
                self.code = m71;
            }
            

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
        
        #[ink::test]
        fn test_init() {
            let ml = Mail::new();
            let sent = ml.get_sent_mail(); //fetching all fund requests
            assert_eq!(sent.len(), 0);           
        }

        #[ink::test]
        fn test_send() {
            let mut ml = Mail::new();
            let sent = ml.get_sent_mail(); //fetching all fund requests
            assert_eq!(sent.len(), 0);   
            
            let accounts = default_accounts();
            set_sender(accounts.bob); 

            let res = ml.send_mail(accounts.eve, "mail1".to_string(), "konnichiwa".to_string());
            assert!(res);
        }


        #[ink::test]
        fn test_get_mask() {
            let mut ml = Mail::new();
            
            let accounts = default_accounts();
            set_sender(accounts.bob); 

            let _res = ml.send_mail(accounts.eve, "mail1".to_string(), "konnichiwa".to_string());
            
            let res = ml.get_algos_and_mask("mail1".to_string());
            let mask1 = res.0;
            assert!(mask1.len() != 0); //implying the mask was fetched 

            set_sender(accounts.eve);
            let res2 = ml.get_algos_and_mask("mail1".to_string());
            let mask2 = res2.0;
            assert_eq!(mask1, mask2); //implying the mask was fetched and also the same for the reciever and sender
        }

        #[ink::test]
        #[should_panic]
        fn test_get_algos_mask_fail() {
            let mut ml = Mail::new();
            
            let accounts = default_accounts();
            set_sender(accounts.bob); 

            let _res = ml.send_mail(accounts.eve, "mail2".to_string(), "konnichiwa".to_string());
            

            set_sender(accounts.alice);
            let _res = ml.get_algos_and_mask("mail2".to_string());
            assert!(true); //implying the mask was fetched since we somehow reached the End of Execution
        }

        #[ink::test]
        fn test_get_algos() {
            let mut ml = Mail::new();
            
            let accounts = default_accounts();
            set_sender(accounts.bob); 

            let _res = ml.send_mail(accounts.eve, "mail2".to_string(), "konnichiwa".to_string());
            

            set_sender(accounts.eve);
            let res = ml.get_algos_and_mask("mail2".to_string());
            let algos = res.1;
            assert!(algos[0] + algos[1] + algos[2] <= 6); //implying the mask was fetched since we somehow reached the End of Execution
        }

        #[ink::test]
        #[should_panic]
        fn test_get_algos_fail() {
            let mut ml = create_contract(1000);
            
            let accounts = default_accounts();
            set_sender(accounts.bob); 

            let _res = ml.send_mail(accounts.eve, "mail2".to_string(), "konnichiwa".to_string());
            

            set_sender(accounts.alice);
            let _res = ml.get_algos_and_mask("mail2".to_string());
            assert!(true); //implying the mask was fetched since we somehow reached the End of Execution
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
