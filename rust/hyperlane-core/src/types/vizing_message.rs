use ethers_core::utils::keccak256;
use serde::de::value;
use sha3::{digest::Update, Digest, Keccak256};
use std::fmt::{Debug, Display, Formatter};

use crate::utils::fmt_domainu64;
use crate::{Decode, Encode, HyperlaneProtocolError, Sequenced, H160, H256, U256};

pub type RawVizingMessage = Vec<u8>;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct VizingMessage {
    /// 4   user nonce
    pub nonce: u32,
    /// 8  Earliest arrival timestamp
    pub earlistarrivaltimestamp: u64,
    /// 8  Latest arrival timestamp
    pub latestarrivaltimestamp: u64,
    /// 20 Relayer address
    pub relayer: H160,
    /// 20 Sender address
    pub sender: H160,
    /// 32  Value from the sender
    pub value: U256,
    /// 8   Destination domain ID
    pub destination: u64,
    /// 0+  Additional parameters
    pub aditionparams: Vec<u8>,
    /// 0+  Message contents
    pub message: Vec<u8>,
}

impl Default for VizingMessage {
    fn default() -> Self {
        Self {
            nonce: 0,
            destination: 0,
            earlistarrivaltimestamp: 0,
            latestarrivaltimestamp: 0,
            relayer: H160::zero(),
            sender: H160::zero(),
            value: U256::zero(),
            aditionparams: vec![],
            message: vec![],
        }
    }
}

impl VizingMessage {
    pub fn build(
        nonce: u32,
        destination: u64,
        earlistarrivaltimestamp: u64,
        latestarrivaltimestamp: u64,
        relayer: H160,
        sender: H160,
        value: U256,
        aditionparams: Vec<u8>,
        message: Vec<u8>,
    ) -> Self {
        Self {
            nonce,
            destination,
            earlistarrivaltimestamp,
            latestarrivaltimestamp,
            relayer,
            sender,
            value,
            aditionparams,
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
            "VizingMessage {{id:{}, nonce: {}, destination: {}, earlistarrivaltimestamp: {}, latestarrivaltimestamp: {}, relayer: {}, sender: {}, value: {}, aditionparams: 0x{}, message: 0x{} }}",
            self.id(),
            self.nonce,
            fmt_domainu64(self.destination),
            self.earlistarrivaltimestamp,
            self.latestarrivaltimestamp,
            self.relayer,
            self.sender,
            self.value,
            hex::encode(&self.aditionparams),
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
        writer.write_all(&self.nonce.to_be_bytes())?;
        writer.write_all(&self.earlistarrivaltimestamp.to_be_bytes())?;
        writer.write_all(&self.latestarrivaltimestamp.to_be_bytes())?;
        writer.write_all(self.relayer.as_fixed_bytes())?;
        writer.write_all(self.sender.as_fixed_bytes())?;
        writer.write_all(value_bytes.as_ref())?;
        writer.write_all(&self.destination.to_be_bytes())?;
        writer.write_all(&self.aditionparams)?;
        writer.write_all(&self.message)?;
        Ok(4 + 8 + 8 + 20 + 20 + 32 + 8 + self.aditionparams.len() + self.message.len())
    }
}

impl Decode for VizingMessage {
    fn read_from<R>(reader: &mut R) -> Result<Self, HyperlaneProtocolError>
    where
        R: std::io::Read,
    {
        let mut nonce = [0u8; 4];
        reader.read_exact(&mut nonce)?;

        let mut earlistarrivaltimestamp = [0u8; 8];
        reader.read_exact(&mut earlistarrivaltimestamp)?;

        let mut latestarrivaltimestamp = [0u8; 8];
        reader.read_exact(&mut latestarrivaltimestamp)?;

        let mut relayer = H160::zero();
        reader.read_exact(relayer.as_mut())?;

        let mut sender = H160::zero();
        reader.read_exact(sender.as_mut())?;

        let mut value_bytes = [0u8; 32];
        reader.read_exact(&mut value_bytes)?;
        let value = U256::from_big_endian(&value_bytes);

        let mut destination = [0u8; 8];
        reader.read_exact(&mut destination)?;

        let mut aditionparams = vec![];
        reader.read_to_end(&mut aditionparams)?;

        let mut message = vec![];
        reader.read_to_end(&mut message)?;

        Ok(Self {
            nonce: u32::from_be_bytes(nonce),
            destination: u64::from_be_bytes(destination),
            earlistarrivaltimestamp: u64::from_be_bytes(earlistarrivaltimestamp),
            latestarrivaltimestamp: u64::from_be_bytes(latestarrivaltimestamp),
            relayer,
            sender,
            value,
            aditionparams,
            message,
        })
    }
}

impl VizingMessage {
    pub fn id(&self) -> H256 {
        // ethers encode & keccak256
        let mut encoded = Vec::new();
        self.write_to(&mut encoded).unwrap();
        let hash = keccak256(&encoded);
        hash.into()
    }
}
