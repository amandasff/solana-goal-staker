use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgUvnMmYVM7g");

#[program]
pub mod goal_staker {
    use super::*;

    pub fn initialize_goal(ctx: Context<InitializeGoal>, amount: u64) -> Result<()> {
        let goal = &mut ctx.accounts.goal;
        goal.user = ctx.accounts.user.key();
        goal.amount = amount;
        goal.status = "pending".to_string();

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.escrow.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.escrow.to_account_info(),
            ],
        )?;
        Ok(())
    }

    pub fn resolve_goal(ctx: Context<ResolveGoal>, success: bool) -> Result<()> {
        let amount = ctx.accounts.goal.amount;
        let recipient = if success {
            &ctx.accounts.user
        } else {
            &ctx.accounts.charity
        };

        **ctx.accounts.escrow.try_borrow_mut_lamports()? -= amount;
        **recipient.try_borrow_mut_lamports()? += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeGoal<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub goal: Account<'info, Goal>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: Escrow account
    #[account(mut)]
    pub escrow: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResolveGoal<'info> {
    #[account(mut)]
    pub goal: Account<'info, Goal>,
    #[account(mut)]
    pub escrow: UncheckedAccount<'info>,
    #[account(mut)]
    pub user: SystemAccount<'info>,
    #[account(mut)]
    pub charity: SystemAccount<'info>,
}

#[account]
pub struct Goal {
    pub user: Pubkey,
    pub amount: u64,
    pub status: String,
}
