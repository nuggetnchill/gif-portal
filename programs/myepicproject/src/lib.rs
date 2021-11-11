use anchor_lang::prelude::*;

declare_id!("3fbYXLkGv4Z6W9gQ7uYjXPq21VRLECYsGMLveVVm8rqc");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
//    Get reference to the account
      let base_account = &mut ctx.accounts.base_account;
//    Init total_gifs
      base_account.total_gifs = 0;
    Ok(())
  }

  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
      let base_account = &mut ctx.accounts.base_account;
//    ItemStruct - for each gif and the data for it
      let item = ItemStruct {
          gif_link: gif_link.to_string(),
          user_address: *base_account.to_account_info().key
//        add votes here
//        votes: 0,
      };
//    Add "item" to gif_list Vec
      base_account.gif_list.push(item);
      base_account.total_gifs += 1;
    
      Ok(())
  }
  
  pub fn update_item(ctx: Context<UpdateItem>, index: u64, vote: Vote) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    
    let i = index as usize;
    if i < base_account.gif_list.len() {
      let mut item = &mut base_account.gifList[i];
      item.votes += vote as i64;
    }
    
    Ok(())
  }
  
  pub fn up_vote(ctx: Context<UpdateItem>, index: u64) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    
    let i = index as usize;
    if i < base_account.gif_list.len() {
      let mut item = &mut base_account.gif[i];
      item.votes += 1;
    }
    
    Ok(())
  }
  
    pub fn down_vote(ctx: Context<UpdateItem>, index: u64) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;

    let i = index as usize;
    if i < base_account.gif_list.len() {
      let mut item = &mut base_account.gif_list[i];
      item.votes -= 1;
    }

    Ok(())
  }
}

// Attach certain variables to the StartStuffOff Context
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space =9000)]
    pub base_account: Account<'info, BaseAccount>,
  
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Specify what data is in AddGif Context
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct UpdateItem<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}

// Create a custom struct to work with
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub votes: i64,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum Vote {
  Up = 1,
  Down = -1,
}

// Tell Solana what to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
//  Attach a Vector of type ItemStruct to account 
    pub gif_list: Vec<ItemStruct>,
}
