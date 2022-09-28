
#![cfg_attr(not(feature = "std"), no_std)]
type Balance = u64;
#[ink::contract]
mod erc20 {
    use ink_env::AccountId;
    use ink_env::call::Call;
    use ink_storage::Mapping;
    use crate::Balance;
    use ink_lang::codegen::StaticEnv;
    use ink_env::*;

    #[ink(stroage)]
    #[derive(Default)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficentBalance,
        InsufficentAllowance,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env.caller();
            balances.insert(&caller, &total_supply);
            // Self::env.emit_event( Transfer {
            //     from: None,
            //     to: Some(caller),
            //     value: total_supply
            // });
            Self {
                total_supply,
                balances,
                allowances: Default::default()
            }
        }

        #[link(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        #[ink(message)]
        pub fn approval_of(&self,
                           owner: AccountId,
                           spender: AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()

        }

        // fn allowance_impl(&mut self, owner: AccountId, spender: AccountId) -> Result<()> {
        //
        // }

        #[ink(message)]
        pub fn approval(&mut self,
                        spender: AccountId,
                        value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), &value);
            // self.env.emit_event( Approval {
            //     owner,
            //     spender,
            //     value
            // });
            Ok(())
        }

        #[ink(message)]
        pub fn transfer(&mut self,
                        to: AccountId,
                        value: Balance) -> Result<()> {
            // let from = self.env().caller();
            // self.transfer_from_to(from, to, value)?;
            Ok(())
        }

        #[ink(message)]
        fn transfer_from_to(&mut self,
                            from: AccountId,
                            to: AccountId,
                            value: Balance) -> Result<()> {
            let from_balance = self.balance_of(from);
            // 验证余额
            if from_balance < value {
                return Err(Error::InsufficentBalance)
            }
            // 验证授权额度
            let from_allowance = self.approval_of(from, to);
            if from_allowance < value {
                return Err(Error::InsufficentAllowance)
            }
            self.balances.insert(from, &(from_balance - value));
            self.approval(from, value)?;
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &to_balance);
            // self.env().emit_event( Transfer {
            //     from: Some(from),
            //     to: Some(to),
            //     value
            // });
            // self.env().emit_event(Transfer {
            //     from: Some(*from),
            //     to: Some(*to),
            //     value,
            // });
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {

    }
}