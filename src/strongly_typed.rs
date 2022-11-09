#![allow(dead_code)]
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

use crate::{AccountConstraintError, AccountConstraintResult};

impl From<AccountConstraintError> for ProgramError {
    fn from(e: AccountConstraintError) -> Self {
        use AccountConstraintError::*;
        let code = match e {
            AccountMissing(_) => 0,
            AccountShouldBeSigner(_) => 1,
            AccountShouldBeWritable(_) => 2,
        };
        ProgramError::Custom(code)
    }
}

struct WritableSignerAccountInfo<'info>(&'info AccountInfo<'info>);

impl<'info> std::ops::Deref for WritableSignerAccountInfo<'info> {
    type Target = AccountInfo<'info>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> TryFrom<&'a AccountInfo<'a>> for WritableSignerAccountInfo<'a> {
    type Error = AccountConstraintError;

    fn try_from(account_info: &'a AccountInfo<'a>) -> Result<Self, Self::Error> {
        if !account_info.is_writable {
            return Err(AccountConstraintError::AccountShouldBeWritable(
                account_info.key.to_string(),
            ));
        }

        if !account_info.is_signer {
            return Err(AccountConstraintError::AccountShouldBeSigner(
                account_info.key.to_string(),
            ));
        }

        Ok(Self(account_info))
    }
}

pub struct VerifiedAccounts<'info> {
    writable_signer: WritableSignerAccountInfo<'info>,
}

impl<'info> VerifiedAccounts<'info> {
    pub fn new(accounts: &'info [AccountInfo<'info>]) -> AccountConstraintResult<Self> {
        let mut accounts_iter = accounts.iter();

        let next = accounts_iter
            .next()
            .ok_or_else(|| AccountConstraintError::AccountMissing("writable_signer".to_string()))?;
        let writable_signer = WritableSignerAccountInfo::try_from(next)?;

        Ok(Self { writable_signer })
    }
}

#[cfg(test)]
mod tests {
    use solana_program::{pubkey::Pubkey, stake_history::Epoch};

    use super::*;
    #[test]
    fn writable_signer() {
        let pubkey = Pubkey::new_unique();
        let mut lamports = 1;
        let account_info = AccountInfo::new(
            &pubkey,
            true,
            true,
            &mut lamports,
            &mut [],
            &pubkey,
            false,
            Epoch::default(),
        );
        let res: AccountConstraintResult<WritableSignerAccountInfo> =
            WritableSignerAccountInfo::try_from(&account_info);
        assert!(res.is_ok());
    }

    #[test]
    fn writable_signer_via_verified_accounts() {
        let pubkey = Pubkey::new_unique();
        let mut lamports = 1;
        let account_info = AccountInfo::new(
            &pubkey,
            true,
            true,
            &mut lamports,
            &mut [],
            &pubkey,
            false,
            Epoch::default(),
        );
        let accounts = vec![account_info];
        let res = VerifiedAccounts::new(&accounts);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().writable_signer.key, &pubkey);
    }

    #[test]
    fn writable_signer_not_writable_via_verified_accounts() {
        let pubkey = Pubkey::new_unique();
        let mut lamports = 1;
        let account_info = AccountInfo::new(
            &pubkey,
            true,
            false,
            &mut lamports,
            &mut [],
            &pubkey,
            false,
            Epoch::default(),
        );
        let accounts = vec![account_info];
        let res = VerifiedAccounts::new(&accounts);
        assert!(res.is_err());
    }
}
