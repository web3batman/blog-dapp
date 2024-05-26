use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_blog {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

pub fn init_blog(ctx: Context<InitBlog>) -> ProgramResult {
    // get accounts from ctx
    let blog_account = &mut ctx.accounts.blog_account;
    let genesis_post_account = &mut ctx.accounts.genesis_post_account;
    let authority = &mut ctx.accounts.authority;

    // sets the blog state
    blog_account.authority = authority.key();
    blog_account.current_post_key = genesis_post_account.key();

    Ok(())
}

// define ctx type
#[derive(Accounts)]
pub struct InitBlog {
    #[account(init, payer = authority, space = 8 + 32 + 32)]
    pub blog_account: Account<'info, BlogState>,
    #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 8)]
    pub genesis_post_account: Account<'info, PostState>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// from pseudo blog state
#[account]
pub struct BlogState {
    pub current_post_key: Pubkey,
    pub authority: Pubkey,
}

pub fn signup_user(ctx: Context<SignupUser>, name: String, avatar: String) -> ProgramResult {
    let user_account = &mut ctx.accounts.user_account;
    let authority = &mut ctx.accounts.authority;

    user_account.name = name;
    user_account.avatar = avatar;
    user_account_authority = authority.key();

    Ok(())
}

// define ctx type
#[derive(Accounts)]
pub struct SignupUser<'info> {
    #[account(init, payer = authority, space = 8 + 40 + 120 + 32)]
    pub user_account: Account<'info, UserState>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserState {
    pub name: String,
    pub avatar: String,
    pub authority: Pubkey,
}

// allows the user to update name and avatar
pub fn update_user(ctx: Context<UpdateUser>, name: String, avatar: String) -> ProgramResult {
    let user_account = &mut ctx.accounts.user_account;

    user_account.name = name;
    user_account.avatar = avatar;

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    #[account(mut, has_one = authority)]
    pub user_account: Account<'info, UserState>,
    pub authority: Signer<'info>,
}

pub fn create_post(ctx: Context<CreatePost>, title: String, content: String) -> ProgramResult {
    let blog_account = &mut ctx.accounts.blog_account;
    let post_account = &mut ctx.accounts.post_account;
    let user_account = &mut ctx.accounts.user_account;
    let authority = &mut ctx.accounts.authority;

    post_account.title = title;
    post_account.content = content;
    post_account.user = user_account.key();
    post_account.timestamp = timestamp();
    post_account.authority = authority.key();
    post_account.pre_post_key = blog_account.current_post_key;

    // store created post id as current post id in blog account
    blog_account.current_post_key = post_account.key();

    Ok(())
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(init, payer = authority, space = 8 + 50 + 500 + 32 + 32 + 32)]
    pub post_account: Account<'info, PostState>,
    #[account(mut, has_one = authority)]
    pub user_account: Account<'info, UserState>,
    #[account(mut)]
    pub blog_account: Account<'info, BlogState>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PostState {
    title: String, // title of the post
    content: String, // content of the post
    user: Pubkey, // user who created the post
    timestamp: u64, // timestamp of the post
    pub pre_post_key: Pubkey, // previous post key
    pub authority: Pubkey, // authority of the post
}

// event post created
#[event]
pub struct PostEvent {
    pub label: String, // label is like 'CREATE', 'UPDATE', 'DELETE'
    pub post_id: Pubkey, // created post id
    pub next_post_id: Pubkey, // next post id - we will use this when we emit delete event
}

pub fn create_post(ctx: Context<CreatePost>, title: String, content: String) -> ProgramResult {
    let post_account = &mut ctx.accounts.post_account;

    emit!(PostEvent {
        label: "CREATE".to_string(),
        post_id: post_account.key(),
        next_post_id: None // same as null
    });

    Ok(())
}

// allows the user to update title and content
pub fn update_post(ctx: Context<UpdatePost>, title: String, content: String) -> ProgramResult {
    let post_account = &mut ctx.accounts.post_account;

    post_account.title = title;
    post_account.content = content;

    emit!(PostEvent {
        label: "UPDATE".to_string(),
        post_id: post_account.key(),
        next_post_id: None // same as null
    });

    Ok(())
}

#[derive(Accounts)]
pub struct UpdatePost<'info> {
    #[account(mut, has_one = authority)]
    pub post_account: Account<'info, PostState>,
    pub authority: Signer<'info>,
}

// allows the user to delete the post
// we need two post account, current_post and next_post account.
// we get pre_post of current_post from current_post and link it to next_post
pub fn delete_post(ctx: Context<DeletePost>) -> ProgramResult {
    let post_account = &mut ctx.accounts.post_account;
    let next_post_account = &mut ctx.accounts.next_post_account;

    next_post_account.pre_post_key = post_account.pre_post_key;

    emit!(PostEvent {
        label: "DELETE".to_string(),
        post_id: post_account.key(),
        next_post_id: Some(next_post_account.key())
    })

    Ok(())
}

// define ctx type for delete_post
#[derive(Accounts)]
pub struct DeletePost<'info> {
    #[account(
        mut,
        has_one = authority,
        close = authority,
        constraint: post_account.key() == next_post_account.pre_post_key)
    ]
    pub post_account: Account<'info, PostState>,
    #[account(mut)]
    pub next_post_account: Account<'info, PostState>,
    pub authority: Signer<'info>,
}

// allows the user to delete the last post
pub fn delete_latest_post(ctx: Context<DeleteLatestPost>) -> ProgramResult {
    let post_account = &mut ctx.accounts.post_account;
    let blog_account = &mut ctx.accounts.blog_account;

    blog_account_current_post_key = post_account.pre_post_key;

    emit!(PostEvent {
        label: "DELETE".to_string(),
        post_id: post_account.key(),
        next_post_id: None // same as null
    });

    Ok(())
}

// define ctx type for delete_latest_post
#[derive(Accounts)]
pub struct DeleteLatestPost<'info> {
    #[account(mut, has_one = authority, close = authority)]
    pub post_account: Account<'info, PostState>,
    #[account(mut)]
    pub blog_account: Account<'info, BlogState>,
    pub authority: Signer<'info>,
}
