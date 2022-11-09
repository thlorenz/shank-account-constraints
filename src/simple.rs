#![allow(dead_code)]
use std::{error::Error, fmt::Display};
// -----------------
// Builtin
// -----------------
#[derive(Debug)]
pub enum ShankAccountCheckError {
    MissingAccountCheck(String),
}

impl Error for ShankAccountCheckError {}
impl Display for ShankAccountCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ShankAccountCheckError::*;
        match self {
            MissingAccountCheck(account) => write!(f, "Account {} was never checked", account),
        }
    }
}

pub type AccountCheckResult<Err> = Result<(), Err>;
trait ChecksAccounts<Err> {
    fn verify_checks(&self) -> AccountCheckResult<Err>;
}

struct AccountChecksForInstructionFoo {
    checked_system_account: bool,
}

impl ChecksAccounts<ShankAccountCheckError> for AccountChecksForInstructionFoo {
    fn verify_checks(&self) -> AccountCheckResult<ShankAccountCheckError> {
        if !self.checked_system_account {
            return Err(ShankAccountCheckError::MissingAccountCheck(
                "system account".to_string(),
            ));
        }
        Ok(())
    }
}

// -----------------
// Custom
// -----------------
#[derive(Debug)]
pub enum CustomAccountCheckError {
    MissingAccountCheck(String),
}

impl Error for CustomAccountCheckError {}
impl Display for CustomAccountCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CustomAccountCheckError::*;
        match self {
            MissingAccountCheck(account) => write!(f, "Account {} was never checked", account),
        }
    }
}
impl ChecksAccounts<CustomAccountCheckError> for AccountChecksForInstructionFoo {
    fn verify_checks(&self) -> AccountCheckResult<CustomAccountCheckError> {
        todo!()
    }
}

impl AccountChecksForInstructionFoo {}
