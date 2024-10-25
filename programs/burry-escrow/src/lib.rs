pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::{deposit::*, withdraw::*};
pub use state::*;

declare_id!("13JoVvwdMBjGZx1AaSCaUmVu6eotFRARegzXWsVPkUH2");

#[program]
pub mod burry_escrow {
    use super::*;
 
    pub fn deposit(ctx: Context<Deposit>, escrow_amount: u64, unlock_price: f64) -> Result<()> {
        deposit_handler(ctx, escrow_amount, unlock_price)
    }
 
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw_handler(ctx)
    }
}
