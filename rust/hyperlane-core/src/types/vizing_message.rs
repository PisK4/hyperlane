use ethers_core::utils::keccak256;
use std::fmt::{Debug, Display, Formatter};

use crate::utils::fmt_domainu64;
use crate::{Decode, Encode, HyperlaneProtocolError, Sequenced, H160, H256, U256};


/// Vizing Omni-Chain Message
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct VizingMessage {
    /// 8  Earliest arrival timestamp
    pub earlistarrivaltimestamp: u64,
    /// 8  Latest arrival timestamp
    pub latestarrivaltimestamp: u64,
    /// 20 Relayer address
    pub relayer: H160,
    /// 8  Destination chain id
    pub destination: u64,
    // /// 0+  Additional parameters
    // pub aditionparams: Vec<u8>,

    /* message id encode start */
    /// 8  Source chain id
    pub origin: u64,
    /// 4   user nonce
    pub nonce: u32,
    /// 32  Source transaction hash
    pub srctxhash: H256,
    /// 20 Sender address
    pub sender: H160,
    /// 32  Value from the sender
    pub value: U256,
    /// 0+  Message contents
    pub message: Vec<u8>,
    /* message id encode end */
}



impl Default for VizingMessage {
    fn default() -> Self {
        Self {
            earlistarrivaltimestamp: 0,
            latestarrivaltimestamp: 0,
            relayer: H160::zero(),
            destination: 0,
            // aditionparams: vec![],
            origin: 0,
            nonce: 0,
            srctxhash: H256::zero(),
            sender: H160::zero(),
            value: U256::zero(),
            message: vec![],
        }
    }
}
impl VizingMessage {
    /// build vizing message via Event Log
    pub fn build(
        earlistarrivaltimestamp: u64,
        latestarrivaltimestamp: u64,
        relayer: H160,
        destination: u64,
        // aditionparams: Vec<u8>,
        origin: u64,
        nonce: u32,
        srctxhash: H256,
        sender: H160,
        value: U256,
        message: Vec<u8>,
    ) -> Self {
        Self {
            earlistarrivaltimestamp,
            latestarrivaltimestamp,
            relayer,
            destination,
            // aditionparams,
            origin,
            nonce,
            srctxhash,
            sender,
            value,
            message,
        }
    }
}

impl Sequenced for VizingMessage {
    fn sequence(&self) -> u32 {
        self.nonce
    }
}

impl Debug for VizingMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VizingMessage {{id:{}, earlistarrivaltimestamp: {}, latestarrivaltimestamp: {}, relayer: {}, destination: {}, origin: {}, nonce: {}, srctxhash: {}, sender: {}, value: {}, message: 0x{} }}",
            self.id(),
            self.earlistarrivaltimestamp,
            self.latestarrivaltimestamp,
            self.relayer,
            fmt_domainu64(self.destination),
            // hex::encode(&self.aditionparams),
            fmt_domainu64(self.origin),
            self.nonce,
            self.srctxhash,
            self.sender,
            self.value,
            hex::encode(&self.message)
        )
    }
}

impl Display for VizingMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "VizingMessage {{ nonce: {}, .. }}", self.nonce)
    }
}

impl Encode for VizingMessage {
    fn write_to<W>(&self, writer: &mut W) -> std::io::Result<usize>
    where
        W: std::io::Write,
    {
        let mut value_bytes = [0u8; 32];
        self.value.to_big_endian(&mut value_bytes);
        writer.write_all(&self.earlistarrivaltimestamp.to_be_bytes())?;
        writer.write_all(&self.latestarrivaltimestamp.to_be_bytes())?;
        writer.write_all(self.relayer.as_fixed_bytes())?;
        writer.write_all(&self.destination.to_be_bytes())?;
        // writer.write_all(&self.aditionparams)?;
        writer.write_all(&self.origin.to_be_bytes())?;
        writer.write_all(&self.nonce.to_be_bytes())?;
        writer.write_all(self.srctxhash.as_fixed_bytes())?;
        writer.write_all(self.sender.as_fixed_bytes())?;
        writer.write_all(value_bytes.as_ref())?;
        writer.write_all(&self.message)?;
        Ok(8 + 8 + 20 + 8 + 8 + 4 + 32 + 20 + 32 + self.message.len())

        // let mut value_bytes = [0u8; 32];
        // self.value.to_big_endian(&mut value_bytes);
        // writer.write_all(&self.nonce.to_be_bytes())?;
        // writer.write_all(&self.earlistarrivaltimestamp.to_be_bytes())?;
        // writer.write_all(&self.latestarrivaltimestamp.to_be_bytes())?;
        // writer.write_all(self.relayer.as_fixed_bytes())?;
        // writer.write_all(self.sender.as_fixed_bytes())?;
        // writer.write_all(value_bytes.as_ref())?;
        // writer.write_all(&self.destination.to_be_bytes())?;
        // writer.write_all(&self.aditionparams)?;
        // writer.write_all(&self.message)?;
        // Ok(4 + 8 + 8 + 20 + 20 + 32 + 8 + self.aditionparams.len() + self.message.len())
    }
}

impl Decode for VizingMessage {
    fn read_from<R>(reader: &mut R) -> Result<Self, HyperlaneProtocolError>
    where
        R: std::io::Read,
    {
        let mut earlistarrivaltimestamp = [0u8; 8];
        reader.read_exact(&mut earlistarrivaltimestamp)?;

        let mut latestarrivaltimestamp = [0u8; 8];
        reader.read_exact(&mut latestarrivaltimestamp)?;

        let mut relayer = H160::zero();
        reader.read_exact(relayer.as_mut())?;

        let mut destination = [0u8; 8];
        reader.read_exact(&mut destination)?;

        // let mut aditionparams = vec![];
        // reader.read_to_end(&mut aditionparams)?;

        let mut origin = [0u8; 8];
        reader.read_exact(&mut origin)?;

        let mut nonce = [0u8; 4];
        reader.read_exact(&mut nonce)?;

        let mut srctxhash = H256::zero();
        reader.read_exact(srctxhash.as_mut())?;

        let mut sender = H160::zero();
        reader.read_exact(sender.as_mut())?;

        let mut value_bytes = [0u8; 32];
        reader.read_exact(&mut value_bytes)?;

        let mut message = vec![];
        reader.read_to_end(&mut message)?;

        Ok(Self {
            earlistarrivaltimestamp: u64::from_be_bytes(earlistarrivaltimestamp),
            latestarrivaltimestamp: u64::from_be_bytes(latestarrivaltimestamp),
            relayer,
            destination: u64::from_be_bytes(destination),
            // aditionparams,
            origin: u64::from_be_bytes(origin),
            nonce: u32::from_be_bytes(nonce),
            srctxhash,
            sender,
            value: U256::from_big_endian(&value_bytes),
            message,
        })
    }
}

impl VizingMessage {
    /// Convert the message to a message id
    pub fn id(&self) -> H256 {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&self.origin.to_be_bytes());
        encoded.extend_from_slice(&self.nonce.to_be_bytes());
        encoded.extend_from_slice(self.srctxhash.as_fixed_bytes());
        encoded.extend_from_slice(self.sender.as_fixed_bytes());
        let mut value_bytes = [0u8; 32];
        self.value.to_big_endian(&mut value_bytes);
        encoded.extend_from_slice(&value_bytes);
        encoded.extend_from_slice(&self.message);

        let hash = keccak256(&encoded);
        hash.into()
    }
}


// mpt_root_new: [u8; 32],
// aggregated_earlist_arrival_timestamp: u64,
// aggregated_latest_arrival_timestamp: u64,
// params: ::std::vec::Vec<InteractionLanding>,

/// Landing Data at destination chain
#[derive(Debug)]
pub struct VizingLandingData {
    /// merkle root of the message tree
    pub mpt_root_new: H256,
    /// Earliest arrival timestamp
    pub aggregated_earlist_arrival_timestamp: u64,
    /// Latest arrival timestamp
    pub aggregated_latest_arrival_timestamp: u64,
    /// params
    pub params: Vec<VizingLandingDataParams>,
}

/// Landing Data at destination chain
#[derive(Debug)]
pub struct VizingLandingDataParams{
    /// Message id
    pub message_id: H256,
    /// Source chain id
    pub src_chain_id: u64,
    /// Source chain nonce
    pub src_chain_nonce: u32,
    /// Source chain tx hash
    pub src_tx_hash: H256,
    /// Sender
    pub sender: H160,
    /// value of native token
    pub value: U256,
    /// cross chain message
    pub message: Vec<u8>,
}

