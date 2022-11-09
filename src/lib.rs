mod simple;
mod strongly_typed;

use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum AccountConstraintError {
    #[error("Account '{0}' was missing from the accounts array")]
    AccountMissing(String),
    #[error("Account '{0}' should be signer")]
    AccountShouldBeSigner(String),
    #[error("Account '{0}' should be writable")]
    AccountShouldBeWritable(String),
}

pub type AccountConstraintResult<T> = Result<T, AccountConstraintError>;
