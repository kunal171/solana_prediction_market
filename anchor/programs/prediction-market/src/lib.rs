use anchor_lang::prelude::*;

declare_id!("EYJtmoTczsfe2cNcWKqpwFckgf47YwiL3dPD26mAcJ3v");

#[program]
pub mod prediction_market {
    use super::*;

    pub fn greet(_ctx: Context<Initialize>) -> Result<()> {
        msg!("GM!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
