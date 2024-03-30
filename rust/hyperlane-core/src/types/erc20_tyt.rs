// use derive_new::new;
use std::io::{Read, Write};

use crate::{Decode, Encode, HyperlaneProtocolError, Sequenced, H160, H256};
pub struct TransferEvent {
    from: H160,
    to: H160,
    value: u128,
}

pub type TransferEventLog = TransferEvent;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Erc20Tyt {
    pub from: H256,
    pub to: H256,
    pub value: u128,
    pub tx_timestamp: u32,
}

impl Erc20Tyt {
    pub fn tx_timestamp(&self) -> u32 {
        self.tx_timestamp
    }
}

impl Sequenced for Erc20Tyt {
    fn sequence(&self) -> u32 {
        self.tx_timestamp
    }
}

impl Erc20Tyt {
    pub fn build(from: H256, to: H256, value: u128, nonce: u32) -> Self {
        Self {
            from,
            to,
            value,
            tx_timestamp: nonce,
        }
    }
}

impl Encode for Erc20Tyt {
    fn write_to<W>(&self, writer: &mut W) -> std::io::Result<usize>
    where
        W: std::io::Write,
    {
        Ok(self.from.write_to(writer)?
            + self.to.write_to(writer)?
            + self.value.write_to(writer)?
            + self.tx_timestamp.write_to(writer)?)
    }
}

impl Decode for Erc20Tyt {
    fn read_from<R>(reader: &mut R) -> Result<Self, HyperlaneProtocolError>
    where
        R: std::io::Read,
        Self: Sized,
    {
        Ok(Self {
            from: H256::read_from(reader)?,
            to: H256::read_from(reader)?,
            value: u128::read_from(reader)?,
            tx_timestamp: u32::read_from(reader)?,
        })
    }
}

// impl Decode for Erc20Tyt {
//     fn read_from<R: Read>(reader: &mut R) -> Result<Self, HyperlaneProtocolError> {
//         let from = H256::read_from(reader)?;
//         let to = H256::read_from(reader)?;
//         let value = u128::read_from(reader)?;
//         let tx_timestamp = u32::read_from(reader)?;
//         Ok(Self {
//             from,
//             to,
//             value,
//             tx_timestamp,
//         })
//     }
// }
