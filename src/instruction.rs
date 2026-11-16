use borsh::BorshDeserialize;

#[derive(Debug, BorshDeserialize)]
pub enum SwapDirection {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, BorshDeserialize)]
pub enum Protocol {
    Pumpfun,
    Pumpswap,
    RaydiumCpmm,
}

#[derive(Debug, BorshDeserialize)]
pub struct SwapArgs {
    pub protocol: Protocol,
    pub direction: SwapDirection,
    pub amount_in: u64,
    pub min_amount_out: u64,
}

#[derive(Debug, BorshDeserialize)]
pub struct MultiHopSwapArgs {
    pub route: Vec<Protocol>,
    pub direction: SwapDirection,
    pub amount_in: u64,
    pub min_amount_out: u64,
}

#[derive(Debug, BorshDeserialize)]
pub enum InstructionData {
    Swap(SwapArgs),
    MultiHopSwap(MultiHopSwapArgs),
}
