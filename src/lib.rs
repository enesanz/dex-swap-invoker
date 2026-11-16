pub mod error;
pub mod instruction;
pub mod processor;

use crate::processor::process_instruction;
use solana_program::entrypoint;

entrypoint!(process_instruction);
