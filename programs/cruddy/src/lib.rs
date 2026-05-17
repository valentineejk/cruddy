use anchor_lang::prelude::*;

declare_id!("Av2mM68jzADnvDLGzea5qAmMfJxR8Aqdd6h3NgRKEzJu");

#[program]
pub mod cruddy {
    use super::*;

    // create
    pub fn create(ctx: Context<Create>, title: String, message: String) -> Result<()> {
        let data = &mut ctx.accounts.journal_entry;
        data.owner = ctx.accounts.owner.key();
        data.title = title;
        data.message = message;
        Ok(())
    }
    // update
    pub fn update(ctx: Context<Update>, title: String, message: String) -> Result<()> {
        let data = &mut ctx.accounts.journal_entry;
        data.message = message;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct Create<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        payer = owner,
        space = 8 + Journal::INIT_SPACE
    )]
    pub journal_entry: Account<'info, Journal>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct Update<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        realloc = 8 + Journal::INIT_SPACE,
        realloc::payer = owner,
        realloc::zero = true

    )]
    pub journal_entry: Account<'info, Journal>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>
}


#[account]
#[derive(InitSpace)]
pub struct Journal {
    pub owner: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(50)]
    pub message: String
}