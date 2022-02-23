#[derive(Accounts)]
#[instruction(token_bump: u8)]
pub struct TestTokenSeedsInit<'info> {
    #[account(
        init,
        seeds = [b"my-token-seed".as_ref()],
        bump = token_bump,
        payer = authority,
        token::mint = mint,
        token::authority = authority,
    )]
    pub my_pda: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub authority: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: AccountInfo<'info>,
}