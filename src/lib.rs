pub mod accounts;
pub mod error;
pub mod instruction;
pub mod instructions;
pub mod processor;
pub mod protocols;
pub mod utils;

use crate::processor::process_instruction;
use solana_program::entrypoint;

entrypoint!(process_instruction);
