use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Swarm {
    pub ai_agent_token: Pubkey,
    pub swarm_token: Pubkey,
    pub owner: Pubkey,
    pub name: String,
    pub description: String,
    pub metadata: String,
    pub members: Vec<Pubkey>,
    pub creation_timestamp: i64,
    pub swarm_id: u64,
}

impl Swarm {
    pub const MAX_SIZE: usize = 32 + // owner
                                4 + 50 + // name (assuming max 50 chars)
                                4 + 200 + // description (assuming max 200 chars)
                                4 + (32 * 100) + // members (assuming max 100 members)
                                8 + // creation_timestamp
                                8; // swarm_id
    pub const PREFIX_SEED: &'static [u8] = b"swarm";
}

#[account]
pub struct AgentToSwarm {
    pub ai_agent: Pubkey,
    pub swarm_id: u64,
    pub status: JoinRequestStatus,
}

impl AgentToSwarm {
    pub const SIZE: usize = 32 + // agent_owner
                           8 + // swarm_id
                           1; // status (enum)
    pub const PREFIX_SEED: &'static [u8] = b"agent_to_swarm";
}

#[account]
#[derive(Default)]
pub struct SwarmCounter {
    pub counter: u64,
}

impl SwarmCounter {
    pub const PREFIX_SEED: &'static [u8] = b"swarm_counter";
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum JoinRequestStatus {
    Pending,
    Approved,
    Declined,
}

impl Default for JoinRequestStatus {
    fn default() -> Self {
        JoinRequestStatus::Pending
    }
}
