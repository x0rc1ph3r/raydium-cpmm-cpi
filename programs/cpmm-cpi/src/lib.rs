use anchor_lang::prelude::*;

pub mod instructions;
use instructions::*;

declare_id!("GnCY3X3sFSTd5tNygUNuKQkXee5CUuJJwPuk4Lfqgjyh");

#[program]
pub mod cp_swap_cpi {
    use super::*;

    pub fn proxy_initialize(
        ctx: Context<ProxyInitialize>,
        init_amount_0: u64,
        init_amount_1: u64,
        open_time: u64,
    ) -> Result<()> {
        instructions::proxy_initialize(ctx, init_amount_0, init_amount_1, open_time)
    }

    pub fn proxy_deposit(
        ctx: Context<ProxyDeposit>,
        lp_token_amount: u64,
        maximum_token_0_amount: u64,
        maximum_token_1_amount: u64,
    ) -> Result<()> {
        instructions::proxy_deposit(
            ctx,
            lp_token_amount,
            maximum_token_0_amount,
            maximum_token_1_amount,
        )
    }
}
