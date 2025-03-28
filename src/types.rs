use crate::support;

pub type AccountId = String;
pub type Balance = u128;
pub type BlockNumber = u32;
pub type Nonce = u32;
pub type Extrinsic = support::Extrinsic<AccountId, support::RuntimeCall>;
pub type Header = support::Header<BlockNumber>;
pub type Block = support::Block<Header, Extrinsic>;
