use anchor_lang::prelude::*;

declare_id!("GUzWCVSiGJ65AEKTXCzu598YVV17apLqivoHZzTF66oa");

#[program]
pub mod anchor_escrow01 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("dont fight me today!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
