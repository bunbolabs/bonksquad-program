use anchor_lang::prelude::*;

declare_id!("8CSHoaVnrsi9TZJd6kw3ihJ5B2baAHQCqScuWHrX2hnb");

#[program]
mod bonksquad {
    use super::*;
    pub fn create_squad(ctx: Context<CreateSquad>, name: String) -> Result<()> {
        let squad_account = &mut ctx.accounts.squad_account;
        squad_account.name = name;
        squad_account.level = 1;
        squad_account.score = 0;
        squad_account.xp = 0;
        squad_account.founder = ctx.accounts.signer.key();

        Ok(())
    }

    pub fn close_squad(ctx: Context<CloseSquad>) -> Result<()> {
        Ok(())
    }

    pub fn create_user(ctx: Context<CreateUser>, name: String) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.name = name;
        user_account.level = 1;
        user_account.score = 0;
        user_account.xp = 0;

        Ok(())
    }

    pub fn close_user(ctx: Context<CloseUser>) -> Result<()> {
        Ok(())
    }

    pub fn create_quest(
        ctx: Context<CreateQuest>,
        score: u16,
        parent: Pubkey,
        quality: u8,
    ) -> Result<()> {
        let quest_account = &mut ctx.accounts.quest_account;
        quest_account.score = score;
        quest_account.parent = parent;
        quest_account.quality = quality;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct CreateSquad<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 4 + 200 + 2 + 2 + 4 + 200 + 2, 
        seeds = [b"create-squad", signer.key().as_ref()], 
        bump,
    )]
    pub squad_account: Account<'info, Squad>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseSquad<'info> {
    #[account(
        mut,
        close = signer,
        seeds = [b"create-squad", signer.key().as_ref()], 
        bump,
    )]
    pub squad_account: Account<'info, Squad>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 4 + 200 + 2 + 2 + 4 + 200 + 2, 
        seeds = [b"create-user", signer.key().as_ref()], 
        bump,
    )]
    pub user_account: Account<'info, User>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseUser<'info> {
    #[account(
        mut,
        close = signer,
        seeds = [b"create-user", signer.key().as_ref()], 
        bump,
    )]
    pub user_account: Account<'info, User>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateQuest<'info> {
    #[account(
        init,
        payer = signer,
        space = 8  + 2 + 32 + 1,
    )]
    pub quest_account: Account<'info, Quest>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct User {
    name: String,
    level: u16,
    score: u16,
    badge: String,
    xp: u16,
    squad: Pubkey,
}

#[account]
pub struct Squad {
    name: String,
    level: u16,
    score: u16,
    badge: String,
    xp: u16,
    founder: Pubkey,
}

#[account]
pub struct Quest {
    score: u16,
    parent: Pubkey,
    quality: u8,
}
