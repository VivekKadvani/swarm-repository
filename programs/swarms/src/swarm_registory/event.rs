use anchor_lang::prelude::*;
use crate::JoinRequestStatus;
// Events

#[event]
pub struct SwarmCreatedEvent {
    pub swarm_id: u64,
    pub owner: Pubkey,
    pub name: String,
    pub timestamp: i64,
}

#[event]
pub struct JoinSwarmEvent {
    pub swarm_id: u64,
    pub ai_agent: Pubkey,
    pub status: JoinRequestStatus,
    pub timestamp: i64,
}

#[event]
pub struct SwarmUpdatedEvent {
    pub swarm_id: u64,
    pub name: String,
    pub description: String,
    pub metadata: String,
    pub timestamp: i64,
}

#[event]
pub struct LeaveSwarmEvent {
    pub swarm_id: u64,
    pub ai_agent: Pubkey,
    pub timestamp: i64,
}
