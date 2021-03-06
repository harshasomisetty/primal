use anchor_lang::prelude::*;
// use anchor_spl::{
//     self,
//     associated_token::AssociatedToken,
//     token::{self, Mint, MintTo, Token, TokenAccount, Transfer},
// };
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use rust_decimal::prelude::*;
use solana_program;
use solana_program::{
    // clock::Clock,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke,
    program::invoke_signed,
    program_option::COption,
    system_instruction,
};

declare_id!("2tJd7McBQbhwCC6pddDxtHq5M2KAEVKXKKxwusKTsSPP");

#[program]
pub mod primal {
    use super::*;

    // TODO Fix imports by adding to  instructions
    pub fn init_treasury(
        ctx: Context<InitTreasury>,
        name: String,
        primal_members: u8,
        starting_price: u8,
        initial_cashout: u8,
    ) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury_account;
        treasury.name = name;
        treasury.primal_members = primal_members;
        treasury.starting_price = starting_price;
        treasury.initial_cashout = initial_cashout;
        treasury.preseed_status = true;

        // start tracking mint
        Ok(())
    }

    pub fn deposit_treasury(
        ctx: Context<DepositTreasury>,
        treasury_bump: u8,
        amount: u64,
    ) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury_account;
        let depositor = &mut ctx.accounts.depositor;

        let creator = &mut ctx.accounts.creator;
        let core_mint = &mut ctx.accounts.core_mint;
        let core_deposit_wallet = &mut ctx.accounts.core_deposit_wallet;
        let system_program = &mut ctx.accounts.system_program;
        let deposit_map = &mut ctx.accounts.deposit_map;

        invoke(
            &system_instruction::transfer(
                &depositor.to_account_info().key,
                &treasury.to_account_info().key,
                amount,
            ),
            &[
                depositor.to_account_info().clone(),
                treasury.to_account_info().clone(),
                system_program.to_account_info().clone(),
            ],
        )?;

        msg!("treasury {}", treasury.to_account_info().key);

        if treasury.preseed_status {
            let depositor_history = &deposit_map.deposit_amount;

            if depositor_history == &(0 as u64) {
                // If the depositor has not previously sent money to treasury.

                msg!("new depositor!");
                anchor_spl::token::mint_to(
                    CpiContext::new_with_signer(
                        ctx.accounts.token_program.to_account_info(),
                        anchor_spl::token::MintTo {
                            mint: core_mint.to_account_info(),
                            to: core_deposit_wallet.to_account_info(),
                            authority: treasury.to_account_info(),
                        },
                        &[&[
                            b"treasury_account".as_ref(),
                            creator.key.as_ref(),
                            treasury.name.as_ref(),
                            &[treasury_bump],
                        ]],
                    ),
                    1,
                )?;

                anchor_spl::token::freeze_account(CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    anchor_spl::token::FreezeAccount {
                        account: core_deposit_wallet.to_account_info(),
                        mint: core_mint.to_account_info(),
                        authority: treasury.to_account_info(),
                    },
                    &[&[
                        b"treasury_account".as_ref(),
                        creator.key.as_ref(),
                        treasury.name.as_ref(),
                        &[treasury_bump],
                    ]],
                ));
            }

            // Updating the depositor's history of sending money.
            deposit_map.deposit_amount = depositor_history + amount;
        } else {
            // don't give any rewards for donating directly to treasury after preseed
        }

        Ok(())
    }

    pub fn end_preseed(ctx: Context<EndPreseed>, treasury_bump: u8) -> Result<()> {
        let treasury_account = &mut ctx.accounts.treasury_account;
        let creator = &mut ctx.accounts.creator;
        let system_program = &mut ctx.accounts.system_program;

        treasury_account.preseed_status = false;
        // treasury.primal_count;
        // treasury_price;
        // TODO array of accounts to mint to for primal
        // find by seeing who owns coins from the preseed mint
        // .getParsedTokenAccountsByOwner(owner, { mint: mint });
        // this is rpc call, see if possible to do in contract

        // otherwise do through client and rpc, but less web3
        // (with rpc, get list of who donated, then here we can see how much each person donated)

        // TODO mint tokens for primal

        // TODO send signer funds
        // system_instruction::send_10% from treasury to signer
        let pre_treasury_balance = ctx.accounts.treasury_account.to_account_info().lamports();
        let one_percent = ctx.accounts.treasury_account.to_account_info().lamports() / (100 as u64);
        let to_send = one_percent as u64 * (ctx.accounts.treasury_account.initial_cashout as u64);

        // invoke_signed(
        //     &system_instruction::transfer(
        //         &treasury.to_account_info().key,
        //         &creator.to_account_info().key,
        //         10000000 as u64,
        //     ),
        //     &[
        //         treasury.to_account_info().clone(),
        //         creator.to_account_info().clone(),
        //         system_program.to_account_info().clone(),
        //     ],
        //     &[&[
        //         b"treasury_account".as_ref(),
        //         creator.key.as_ref(),
        //         treasury.name.as_ref(),
        //         &[treasury_bump],
        //     ]],
        // )?;

        // add checks like https://hackmd.io/XP15aqlzSbG8XbGHXmIRhg
        **ctx
            .accounts
            .treasury_account
            .to_account_info()
            .try_borrow_mut_lamports()? -= to_send;

        **ctx.accounts.creator_account.try_borrow_mut_lamports()? += to_send;

        // TODO mint tokens for community, proportional to starting price parameter

        let post_treasury_balance = ctx.accounts.treasury_account.to_account_info().lamports();

        msg!(
            "treasury balance before and after sending {}, {}",
            pre_treasury_balance,
            post_treasury_balance
        );

        // TODO init swap

        Ok(())
    }

    pub fn raise_money(ctx: Context<RaiseMoney>) -> Result<()> {
        // mint community tokens

        // sell into market
        Ok(())
    }
}

//space
#[derive(Accounts)]
#[instruction(name: String, primal_members: u8, starting_price: u8, initial_cashout: u8)]
pub struct InitTreasury<'info> {
    #[account(init, payer = creator, space = 8 + std::mem::size_of::<TreasuryAccount>(), seeds = [b"treasury_account".as_ref(), creator.key.as_ref(), name.as_ref()], bump)]
    pub treasury_account: Account<'info, TreasuryAccount>,
    #[account(init, payer = creator, seeds = [b"core_mint".as_ref(), treasury_account.key().as_ref()], bump, mint::decimals = 1, mint::authority = treasury_account, mint::freeze_authority = treasury_account) ]
    pub core_mint: Account<'info, Mint>,
    #[account(init, payer = creator, seeds = [b"primal_mint".as_ref(), treasury_account.key().as_ref()], bump, mint::decimals = 9, mint::authority = treasury_account, mint::freeze_authority = treasury_account)]
    pub primal_mint: Account<'info, Mint>,
    #[account(init, payer = creator, seeds = [b"member_mint".as_ref(), treasury_account.key().as_ref()], bump, mint::decimals = 9, mint::authority = treasury_account, mint::freeze_authority = treasury_account)]
    pub member_mint: Account<'info, Mint>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

// need to have addresses of mints
#[derive(Accounts)]
#[instruction(treasury_bump: u8, amount: u64)]
pub struct DepositTreasury<'info> {
    #[account(mut)]
    pub treasury_account: Account<'info, TreasuryAccount>,
    #[account(init_if_needed, payer = depositor, seeds = [b"deposit_map".as_ref(), treasury_account.key().as_ref(), depositor.key.as_ref()], bump)]
    pub deposit_map: Account<'info, DepositTrack>,

    #[account(mut, constraint = core_mint.mint_authority == COption::Some(treasury_account.key()))]
    pub core_mint: Account<'info, Mint>,
    #[account(init_if_needed, payer = depositor, associated_token::mint = core_mint, associated_token::authority = depositor)]
    pub core_deposit_wallet: Account<'info, TokenAccount>,

    // #[account(mut)]
    // pub primal_mint: Account<'info, Mint>,
    // TODO add wallet for primal
    // #[account(mut)]
    // pub member_mint: Account<'info, Mint>,
    // TODO add wallet for member mint
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub creator: AccountInfo<'info>,
    #[account(mut)]
    pub depositor: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(treasury_bump: u8)]
pub struct EndPreseed<'info> {
    #[account(mut)]
    pub treasury_account: Account<'info, TreasuryAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub creator_account: AccountInfo<'info>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RaiseMoney<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
}

// TODO add extra space
#[account]
#[derive(Default)]
pub struct TreasuryAccount {
    pub name: String,         // name of project
    pub primal_members: u8,   // Number of primal, or early impactful, investors
    pub starting_price: u8,   // starting price of token when minting share tokens
    pub initial_cashout: u8,  // percentage of treasury creator immediately gets
    pub preseed_status: bool, // If project is currently in preseed stage
}

#[account]
#[derive(Default)]
pub struct DepositTrack {
    pub deposit_amount: u64,
}
