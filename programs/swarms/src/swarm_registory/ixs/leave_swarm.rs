use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::{constants::DEFAULT_OWNER, error::SwarmError, AgentToSwarm, LeaveSwarmEvent, Swarm};

pub fn leave_swarm(ctx: Context<LeaveSwarm>, swarm_id : u64) -> Result<()> {

    require!(
        swarm_id == ctx.accounts.swarm.swarm_id,
        SwarmError::WrongSwarmId
    );
    
    let swarm = &mut ctx.accounts.swarm;
    let ai_agent = ctx.accounts.ai_agent.key();

    // Validate that the agent is a member of the swarm
    let position = swarm.members.iter().position(|&p| p == ai_agent);
    require!(position.is_some(), SwarmError::NotSwarmMember);

    // Remove the agent from the members list
    swarm.members.remove(position.unwrap());

    // The AgentToSwarm account will be closed and lamports returned to the agent owner

    emit!(LeaveSwarmEvent {
        swarm_id,
        ai_agent,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(swarm_id: u64)]
pub struct LeaveSwarm<'info> {
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
         seeds = [b"swarm", swarm_id.to_le_bytes().as_ref()],
        bump
    )]
    pub swarm: Account<'info, Swarm>,

    #[account(
        mut,
        close = ai_agent,
        seeds = [b"agent_to_swarm", ai_agent.key().as_ref(), swarm.key().as_ref()],
        bump
    )]
    pub agent_to_swarm: Account<'info, AgentToSwarm>,

    pub system_program: Program<'info, System>,
}
