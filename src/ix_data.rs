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
    Whirlpool,
}

#[derive(Debug, BorshDeserialize)]
pub struct MultiHopSwapArgs {
    pub hops: Vec<Protocol>,
    pub direction: SwapDirection,
    pub amount_in: u64,
    pub min_amount_out: u64,
}
