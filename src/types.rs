use crate::services::queue_service::QueueHandle;
use alloy::network::Ethereum;
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
};
use alloy::providers::{Identity, RootProvider};

pub type L2Provider = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
    Ethereum,
>;

#[derive(Clone)]
pub struct AppState {
    pub queue: QueueHandle,
    pub provider: L2Provider,
}
