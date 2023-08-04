#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::primitives::AccountId;
use sp_runtime::MultiAddress;

#[derive(scale::Encode)]
enum RuntimeCall {
    #[codec(index = 10)]
    Balances(BalancesCall),
}

#[derive(scale::Encode)]
enum BalancesCall {
    #[codec(index = 0)]
    Transfer {
        dest: MultiAddress<AccountId, ()>,
        #[codec(compact)]
        value: u128,
    },
}

#[ink::contract]
mod runtime_call {
    use crate::{BalancesCall, RuntimeCall};

    use ink::env::Error as EnvError;

    #[ink(storage)]
    #[derive(Default)]
    pub struct RuntimeCaller;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum RuntimeError {
        CallRuntimeFailed,
    }

    impl From<EnvError> for RuntimeError {
        fn from(e: EnvError) -> Self {
            match e {
                EnvError::CallRuntimeFailed => RuntimeError::CallRuntimeFailed,
                _ => panic!("Unexpected error from `pallet-contracts`."),
            }
        }
    }

    impl RuntimeCaller {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            Default::default()
        }

        #[ink(message)]
        pub fn call_nonexistent_extrinsic(&mut self) -> Result<(), RuntimeError> {
            self.env().call_runtime(&()).map_err(Into::into)
        }

        #[ink(message)]
        pub fn transfer_through_runtime(
            &mut self,
            receiver: AccountId,
            value: Balance,
        ) -> Result<(), RuntimeError> {
            self.env()
                .call_runtime(&RuntimeCall::Balances(BalancesCall::Transfer {
                    dest: receiver.into(),
                    value,
                }))
                .map_err(Into::into)
        }
    }
}
