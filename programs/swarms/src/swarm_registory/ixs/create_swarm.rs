use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::{
    constants::DEFAULT_OWNER, error::SwarmError, AgentToSwarm, JoinRequestStatus, Swarm,
    SwarmCounter, SwarmCreatedEvent,
};

pub fn create_swarm(
    ctx: Context<CreateSwarm>,
    name: String,
    description: String,
    metadata: String,
) -> Result<()> {

    let swarm = &mut ctx.accounts.swarm;
    let counter = &mut ctx.accounts.swarm_counter;

    // Increment the counter first to get a unique swarm_id
    counter.counter += 1;
    let swarm_id = counter.counter;

    // Initialize the swarm data
    swarm.ai_agent_token= ctx.accounts.ai_agent.key();
    swarm.swarm_token = ctx.accounts.swarm_token.key();
    swarm.owner = ctx.accounts.owner.key();
    swarm.name = name;
    swarm.description = description;
    swarm.metadata = metadata;
    swarm.creation_timestamp = Clock::get()?.unix_timestamp;
    swarm.swarm_id = swarm_id;

    emit!(SwarmCreatedEvent {
        swarm_id,
        owner: ctx.accounts.owner.key(),
        name: swarm.name.clone(),
        timestamp: swarm.creation_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction( name: String, description: String)]
pub struct CreateSwarm<'info> {
    #[
        account(
            mut,
            constraint = owner.key() == DEFAULT_OWNER @ SwarmError::UnauthorizedOwner)
    ]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub ai_agent: AccountInfo<'info>,

    #[account(mut)]
    pub swarm_token: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = owner,
        space = 32,
        seeds = [SwarmCounter::PREFIX_SEED],
        bump
    )]
    pub swarm_counter: Account<'info, SwarmCounter>,

    #[account(
        init,
        payer = owner,
        space = 8 + Swarm::MAX_SIZE,
        seeds = [Swarm::PREFIX_SEED, (swarm_counter.counter+1).to_le_bytes().as_ref()],
        bump
    )]
    pub swarm: Account<'info, Swarm>,

    pub system_program: Program<'info, System>,
}
