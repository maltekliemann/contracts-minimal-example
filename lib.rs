#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod runtime_call {
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
    }
}
