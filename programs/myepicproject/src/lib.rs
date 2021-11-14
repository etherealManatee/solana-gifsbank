use anchor_lang::prelude::*; //kinda like import, import the tools Anchor provides to make wriing Solana programs easier.

declare_id!("783sLc7iFU7zpV1p9ivoeDqbXSoKzr99DJFLCnBKPpTS");//will change this later, info for solana on how to run the program

#[program] //everything below here is our program
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    //get a reference to the account
    let base_account = &mut ctx.accounts.base_account;
    //initialize total_gifs
    base_account.total_gifs = 0;
    Ok(())
  }

  // Another function woo!
  // The function now accepts a gif_link param from the user. We also reference the user from the Context
  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
    // Get a reference to the account and increment total_gifs.
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    //build struct
    let item = ItemStruct {
        gif_link: gif_link.to_string(),
        user_address: *user.to_account_info().key,
    };


    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
  }
}

//Attach certain variables to the StartStuffOff context
//
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)] //telling solana how we want to initialize BaseAccount
    /*
    init will tell solana to create a new account (like a file) owned by our current program
    payer = user tells our program who's paying for the account to be created. 
    space = 9000 which will allocate 9000 bytes of space for our account

    we are paying for an account because it's like paying for rent
    */
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Specify what data you want in the AddGif Context.
// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

//tell solana what we want to store on this account
//tells our program what kinda of account it can make and what to hold inside of it
//attach a vector of type ItemStruct to the account
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}