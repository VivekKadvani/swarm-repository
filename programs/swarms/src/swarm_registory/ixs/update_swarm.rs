use anchor_lang::prelude::*;
use crate::{
    constants::DEFAULT_OWNER, error::SwarmError, Swarm,SwarmUpdatedEvent
};

// Swarm update parameters
#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct UpdateSwarmInput {
    name: Option<String>,
    description: Option<String>,
    metadata: Option<String>,
}

pub fn update_swarm(
    ctx: Context<UpdateSwarm>,
    input: UpdateSwarmInput,
) -> Result<()> {
    let swarm = &mut ctx.accounts.swarm;
    
    // Only update fields that are provided (not None)
    if let Some(new_name) = input.name {
        swarm.name = new_name;
    }
    
    if let Some(new_description) = input.description {
        swarm.description = new_description;
    }
    
    if let Some(new_metadata) = input.metadata {
        swarm.metadata = new_metadata;
    }
    
    // Emit an event for the update
    emit!(SwarmUpdatedEvent {
        swarm_id: swarm.swarm_id,
        name: swarm.name.clone(),
        description: swarm.description.clone(),
        metadata: swarm.metadata.clone(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
#[derive(Accounts)]
#[instruction(input: UpdateSwarmInput)]
pub struct UpdateSwarm<'info> {
    #[account(
        constraint = owner.key() == DEFAULT_OWNER @ SwarmError::UnauthorizedOwner
    )]
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        seeds = [Swarm::PREFIX_SEED, swarm.swarm_id.to_le_bytes().as_ref()],
        bump,
        constraint = swarm.owner == owner.key() @ SwarmError::NotSwarmOwner
    )]
    pub swarm: Account<'info, Swarm>,
    
    pub system_program: Program<'info, System>,
}