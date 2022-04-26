// use anchor_lang::prelude::*;

// declare_id!("BiHcQCAxV75ziYyyBJmvsK8EjQfEPNBXPoyWfXB8rBuU");

// #[program]
// mod mysolanaapp {
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("BiHcQCAxV75ziYyyBJmvsK8EjQfEPNBXPoyWfXB8rBuU");

#[program]
pub mod solana_trading_app {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}