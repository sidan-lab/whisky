use super::Budget;

#[derive(Clone, Debug, PartialEq)]
pub struct Action {
    pub index: u16,
    pub budget: Budget,
    tag: RedeemerTag,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RedeemerTag {
    Spend,
    Mint,
    Cert,
    Reward,
}
