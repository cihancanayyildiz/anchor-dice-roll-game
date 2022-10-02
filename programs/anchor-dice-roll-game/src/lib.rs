use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod anchor_dice_roll_game {
    use super::*;
    use anchor_lang::solana_program::{ program::invoke, system_instruction::transfer };
    pub fn setup(ctx: Context<Setup>, player: Pubkey, bet_amount: u64) -> Result<()> {
        let dice_roll = &mut ctx.accounts.dice_roll;

        dice_roll.players = [ctx.accounts.user.key(), player];
        dice_roll.bump = *ctx.bumps.get("dice_roll").unwrap();
        dice_roll.bet_amount = bet_amount;
        dice_roll.game_state = String::from("");

        Ok(())
    }

    pub fn play(ctx: Context<Play>, player_choices: String) -> Result<()> {
        let dice_roll = &mut ctx.accounts.dice_roll;

        invoke(
            &transfer(
                ctx.accounts.user.to_account_info().key,
                dice_roll.to_account_info().key,
                dice_roll.bet_amount
            ),
            &[
                ctx.accounts.user.to_account_info(),
                dice_roll.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ]
        )?;

        let mut num_vec: Vec<i32> = Vec::new();

        for item in player_choices.chars() {
            if item != ',' {
                num_vec.push((item as i32) - 0x30);
            }
        }
        let clock: Clock = Clock::get().unwrap();

        // todo: Seed for randomization needs to be improved.
        let dice_result: i32 = ((clock.unix_timestamp % 6) + 1).try_into().unwrap();

        // win
        if num_vec.contains(&dice_result) {
            dice_roll.game_state = format!("You won! Dice result: {}", dice_result);
            //todo: This needs to be tested with frontend and phantom wallet.
            **dice_roll.to_account_info().try_borrow_mut_lamports()? -= dice_roll.bet_amount;
            **ctx.accounts.user.try_borrow_mut_lamports()? += 2 * dice_roll.bet_amount;
        } else {
            dice_roll.game_state = format!("You lose! Dice result: {}", dice_result);
        }

        Ok(())
    }
}
#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(
        init,
        payer = user,
        space = DiceRoll::LEN,
        seeds = ["dice_roll".as_bytes(), user.key().as_ref()],
        bump
    )]
    pub dice_roll: Account<'info, DiceRoll>,
    #[account(mut)]
    pub user: Signer<'info>,
    //pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Play<'info> {
    #[account(
        mut, 
        seeds = ["dice_roll".as_bytes(), user.key().as_ref()],
        bump = dice_roll.bump
    )]
    pub dice_roll: Account<'info, DiceRoll>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct DiceRoll {
    players: [Pubkey; 2],
    bet_amount: u64,
    game_state: String,
    bump: u8,
}

impl DiceRoll {
    const LEN: usize = 200;
}