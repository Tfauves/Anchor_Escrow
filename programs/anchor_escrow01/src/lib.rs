use anchor_lang::prelude::*;

declare_id!("GUzWCVSiGJ65AEKTXCzu598YVV17apLqivoHZzTF66oa");

#[program]
pub mod anchor_escrow01 {
    use super::*;

    // Processor

    const ESCROW_PDA_SEED: &[u8] = b"escrow";

    pub fn initialize(
        ctx: Context<Initialize>,
        _vault_account_bump: u8,
        initializer_amount: u64,
        taker_amount: u64,
    ) -> Result<()> {
        ctx.accounts.escrow_account.initializer_key = *ctx.accounts.initializer.key;
        ctx.accounts.escrow_account.initializer_deposit_token_account = *ctx.accounts
        .initializer_deposit_token_account
        .to_account_info()
        .key;
        ctx.accounts.escrow_account.initializer_receive_token_account = *ctx.accounts
        .initializer_receive_token_account
        .to_account_info()
        .key;
        ctx.accounts.escrow_account.initializer_amount = initializer_amount;
        ctx.accounts.escrow_account.taker_amount = taker_amount;

        let(vault_authority, _vault_authority_bump) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        token::set_authority(
            ctx.accounts.into_set_authority_context(),
            AuthorityType:: AccountOwner,
            Some(vault_authority),
        )?;

        token::transfer(
            ctx.accounts.into_transfer_to_pda_context(),
            ctx.accounts.escrow_account.initializer_amount,
        )?;


        // todo
        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        // todo

        Ok(())
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        // todo

        Ok(())
    }
}

// Depending on the program functions,
// the instructions should bring in the accounts that are needed for operations.
// we have to first consider what data is to be stored in escrow account.

// Instructions
#[derive(Accounts)]
pub struct Initialize<'info> {
    // Signer of InitialEscrow instruction. To be stored in EscrowAccount
    pub initializer: AccountInfo<'info>,
    // The account of token account for token exchange. To be stored in EscrowAccount
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    // The account of token account for token exchange. To be stored in EscrowAccount
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    // The account of TokenProgram
    pub token_program: AccountInfo<'info>,
    // The account of EscrowAccount
    pub escrow_account: Account<'info, EscrowAccount>,
    // The account of Vault, which is created by Anchor via constraints
    pub vault_account: Account<'info, TokenAccount>,
    // Mint
    pub mint: Account<'info, Mint>,
    // System Program
    pub system_program: AccountInfo<'info>,
    // Rent
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    // The initializer of EscrowAccount
    pub initializer: AccountInfo<'info>,
    // The address of token account for token exchange
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    // PDA
    pub vault_account: Account<'info, TokenAccount>,
    // PDA
    pub vault_authority: AccountInfo<'info>,
    // The address of token account for token exchange
    pub escrow_account: Account<'info, EscrowAccount>,
    // The address of TokenProgram
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    // The address of TokenProgram
    pub taker: AccountInfo<'info>,
    // Token account for token exchange
    pub taker_deposit_token_account: Account<'info, TokenAccount>,
    // Token account for token exchange
    pub taker_receive_token_account: Account<'info, TokenAccount>,
    // Token account for token exchange
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    // Token account for token exchange
    pub initializer_recieve_token_account: Account<'info, TokenAccount>,
    // To be used in constraints.
    pub initializer: AccountInfo<'info>,
    // The address of EscrowAccount.
    // Have to check if the EscrowAccount follows certain constraints.
    pub escrow_account: Account<'info, EscrowAccount>,
    // PDA
    pub vault_account: Account<'info, TokenAccount>,
    // PDA
    pub vault_authority: AccountInfo<'info>,
    // The address of the TokenProgram
    pub token_program: AccountInfo<'info>,
}

// Program account
// Accounts owned and managed by the program
// are defined in the #[account] section.

#[account]
pub struct EscrowAccount {
    // To authorize the actions properly
    pub initializer_key: Pubkey,
    // To record the deposit account of initialzer
    pub initializer_deposit_token_account: Pubkey,
    // To record the receiving account of initializer
    pub initializer_receive_token_account: Pubkey,
    // To record how much token should the initializer transfer to taker
    pub initializer_amount: u64,
    // To record how much token the initializer will receive from the taker
    pub taker_amount: u64,
}
