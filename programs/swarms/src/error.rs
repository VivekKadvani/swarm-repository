use anchor_lang::prelude::error_code;

#[error_code]
pub enum SwarmError {
    #[msg("Not the swarm owner")]
    NotSwarmOwner,

    #[msg("Not a swarm member")]
    NotSwarmMember,

    #[msg("Only the default owner can perform this action")]
    UnauthorizedOwner,

    #[msg("Swarm id diiferent from the swarm")]
    WrongSwarmId,
}
