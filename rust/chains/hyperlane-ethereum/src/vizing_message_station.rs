#![allow(clippy::enum_variant_names)]
#![allow(missing_docs)]

use std::collections::HashMap;
use std::num::NonZeroU64;
use std::ops::RangeInclusive;
use std::sync::Arc;

use async_trait::async_trait;
use ethers::abi::AbiEncode;
use ethers::prelude::Middleware;
use ethers_contract::builders::ContractCall;
use tracing::instrument;

use hyperlane_core::{
    utils::bytes_to_hex, ChainCommunicationError, ChainResult, ContractLocator, HyperlaneAbi,
    HyperlaneChain, HyperlaneContract, HyperlaneDomain, HyperlaneMessage, HyperlaneProtocolError,
    HyperlaneProvider, Indexer, LogMeta, Mailbox, RawHyperlaneMessage, SequenceAwareIndexer,
    TxCostEstimate, TxOutcome, VizingMessage, H160, H256, U256,
};

use crate::contracts::arbitrum_node_interface::ArbitrumNodeInterface;
use crate::contracts::i_mailbox::{IMailbox as EthereumMailboxInternal, ProcessCall, IMAILBOX_ABI};
use crate::trait_builder::BuildableWithProvider;
use crate::tx::{call_with_lag, fill_tx_gas_params, report_tx};
use crate::EthereumProvider;

use crate::contracts::message_space_station_ug::{
    LaunchCall, MessageSpaceStationUg as VizingMessageStationInternal, MESSAGESPACESTATIONUG_ABI,
};

impl<M> std::fmt::Display for VizingMessageStationInternal<M>
where
    M: Middleware,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub struct VizingMessageStationBuilder {
    pub reorg_period: u32,
}

#[async_trait]
impl BuildableWithProvider for VizingMessageStationBuilder {
    type Output = Box<dyn SequenceAwareIndexer<VizingMessage>>;

    async fn build_with_provider<M: Middleware + 'static>(
        &self,
        provider: M,
        locator: &ContractLocator,
    ) -> Self::Output {
        Box::new(VizingLaunchMessageIndexer::new(
            Arc::new(provider),
            locator,
            self.reorg_period,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct VizingLaunchMessageIndexer<M>
where
    M: Middleware,
{
    contract: Arc<VizingMessageStationInternal<M>>,
    provider: Arc<M>,
    reorg_period: u32,
}

impl<M> VizingLaunchMessageIndexer<M>
where
    M: Middleware + 'static,
{
    pub fn new(provider: Arc<M>, locator: &ContractLocator, reorg_period: u32) -> Self {
        let contract = Arc::new(VizingMessageStationInternal::new(
            locator.address,
            provider.clone(),
        ));
        Self {
            contract,
            provider,
            reorg_period,
        }
    }

    #[instrument(level = "debug", err, ret, skip(self))]
    async fn get_finalized_block_number(&self) -> ChainResult<u32> {
        Ok(self
            .provider
            .get_block_number()
            .await
            .map_err(ChainCommunicationError::from_other)?
            .as_u32()
            .saturating_sub(self.reorg_period))
    }
}

#[async_trait]
impl<M> Indexer<VizingMessage> for VizingLaunchMessageIndexer<M>
where
    M: Middleware + 'static,
{
    async fn get_finalized_block_number(&self) -> ChainResult<u32> {
        self.get_finalized_block_number().await
    }

    /// Note: This call may return duplicates depending on the provider used
    #[instrument(err, skip(self))]
    async fn fetch_logs(
        &self,
        range: RangeInclusive<u32>,
    ) -> ChainResult<Vec<(VizingMessage, LogMeta)>> {
        let events = self
            .contract
            .successful_launch_message_filter()
            .from_block(*range.start())
            .to_block(*range.end())
            .query_with_meta()
            .await?
            .into_iter()
            .map(|(event, meta)| {
                (
                    VizingMessage::build(
                        event.nonce.into(),
                        event.params.dest_chainld.into(),
                        event.params.earlist_arrival_timestamp.into(),
                        event.params.latest_arrival_timestamp.into(),
                        event.params.relayer.into(),
                        event.params.sender.into(),
                        event.params.value.into(),
                        (&event.params.adition_params.as_ref()).to_vec(),
                        (&event.params.message.as_ref()).to_vec(),
                    ),
                    meta.into(),
                )
            })
            .collect();
        print!("vizing launch: {:?}", events);
        Ok(events)
    }
}

#[async_trait]
impl<M> SequenceAwareIndexer<VizingMessage> for VizingLaunchMessageIndexer<M>
where
    M: Middleware + 'static,
{
    #[instrument(err, skip(self))]
    async fn latest_sequence_count_and_tip(&self) -> ChainResult<(Option<u32>, u32)> {
        let tip = Indexer::<VizingMessage>::get_finalized_block_number(self).await?;
        let sequence = self.contract.nonce().block(u64::from(tip)).call().await?;
        Ok((Some(sequence), tip))
    }
}

pub struct VizingMessageStationAbi;

impl HyperlaneAbi for VizingMessageStationAbi {
    const SELECTOR_SIZE_BYTES: usize = 4;

    fn fn_map() -> HashMap<Vec<u8>, &'static str> {
        super::extract_fn_map(&MESSAGESPACESTATIONUG_ABI)
    }
}
