pub fn create_option(ctx: Context<CreatePost>, trade: u64) -> ProgramResult {

    trade = trade;

    emit!(PostOption {
        assetValue: trade.to_string(),
        expiration: expire.key(),
        seller_id: seller.key(),
        buyer_id: buyer.key()
    });

    Ok(())
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(init, payer = authority, space = 8 + 8 + 8)]
    pub seller: Account<'info, Trader>,
    #[account(mut, has_one = authority)]
    pub buyer: Account<'info, Trader>,
    #[account(mut)]
    pub expire: Singer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Trader {
    pub name: String,
    pub authority: String,
}

#[event]
pub struct PostOption {
    pub assetValue: Unit,
    pub expiration: Uint,
    pub seller_id: Pubkey,
    pub buyer_id: Pubkey,
}