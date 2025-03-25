#![allow(unused)]

use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod swarm_registory;

use swarm_registory::*;

declare_id!("2E2X3oMTJyuJRsbxD5W6VoZCGzAbmc2iYHZCp95PEGH5");

#[program]
pub mod swarms {
    use super::*;

    pub fn create_swarm(
        ctx: Context<CreateSwarm>,
        name: String,
        description: String,
        metadata: String,
    ) -> Result<()> {
        swarm_registory::create_swarm(ctx, name, description, metadata)
    }

    pub fn join_swarm(ctx: Context<RequestJoinSwarm>, swarm_id: u64) -> Result<()> {
        swarm_registory::join_swarm(ctx, swarm_id)
    }

    pub fn leave_swarm(ctx: Context<LeaveSwarm>, swarm_id: u64) -> Result<()> {
        swarm_registory::leave_swarm(ctx, swarm_id)
    }

    pub fn update_swarm(ctx: Context<UpdateSwarm>, input: UpdateSwarmInput) -> Result<()> {
        swarm_registory::update_swarm(ctx, input)
    }

}
