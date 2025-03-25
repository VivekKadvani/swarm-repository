use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::{
    constants::DEFAULT_OWNER, error::SwarmError, AgentToSwarm, JoinRequestStatus, JoinSwarmEvent,
    Swarm,
};

pub fn join_swarm(ctx: Context<RequestJoinSwarm>, swarm_id: u64) -> Result<()> {
    require!(
        swarm_id == ctx.accounts.swarm.swarm_id,
        SwarmError::WrongSwarmId
    );

    let agent_to_swarm = &mut ctx.accounts.agent_to_swarm;
    agent_to_swarm.ai_agent = ctx.accounts.ai_agent.key();
    agent_to_swarm.swarm_id = swarm_id;
    agent_to_swarm.status = JoinRequestStatus::Approved;

    // Add the agent owner to the Swarm's members list
    let swarm = &mut ctx.accounts.swarm;
    swarm.members.push(ctx.accounts.ai_agent.key());

    emit!(JoinSwarmEvent {
        swarm_id,
        ai_agent: ctx.accounts.ai_agent.key(),
        status: JoinRequestStatus::Approved,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(swarm_id: u64)]
pub struct RequestJoinSwarm<'info> {
    #[
        account(
            mut,
            constraint = owner.key() == DEFAULT_OWNER @ SwarmError::UnauthorizedOwner)
    ]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub ai_agent: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [Swarm::PREFIX_SEED, swarm_id.to_le_bytes().as_ref()],
        bump
    )]
    pub swarm: Account<'info, Swarm>,

    #[account(
        init,
        payer = owner,
        space = 8 + AgentToSwarm::SIZE,
        seeds = [AgentToSwarm::PREFIX_SEED, ai_agent.key().as_ref(), swarm.key().as_ref()],
        bump
    )]
    pub agent_to_swarm: Account<'info, AgentToSwarm>,

    pub system_program: Program<'info, System>,
}
